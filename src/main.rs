extern crate bincode;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;

const TWO_PI: f32 = std::f32::consts::PI * 2.0;

#[derive(Serialize)]
struct WaveHeader {
    riff_id: &'static [u8; 4],
    riff_size: u32,
    wavetype: &'static [u8; 4],
    fmt_id: &'static [u8; 4],
    fmt_size: u32,
    fmt_code: u16,
    channels: u16,
    samplerate: u32,
    byterate: u32,
    align: u16,
    bits: u16,
    wave_id: &'static [u8; 4],
    wave_size: u32,
}

fn write_wav(fname: &str, buff: Vec<i16>, dur: u32, sr: u16, ch: u8) {
    let mut wh: WaveHeader = WaveHeader {
        riff_id: b"RIFF",
        riff_size: 0,
        wavetype: b"WAVE",
        fmt_id: b"fmt ",
        fmt_size: 16,
        fmt_code: 1,
        channels: ch as u16,
        samplerate: sr as u32,
        bits: 16,
        align: 0,
        byterate: 0,
        wave_id: b"data",
        wave_size: 0,
    };

    let mut total_samples = sr as u32 * dur;
    let byte_total = total_samples * 2 * ch as u32;
    wh.riff_size = byte_total + std::mem::size_of::<WaveHeader>() as u32;
    wh.wave_size = byte_total;
    wh.align = (wh.channels * wh.bits) / 8;
    wh.byterate = wh.samplerate * wh.align as u32;

    total_samples *= 2;
    let mut end_buffer: Vec<i16> = Vec::new();

    let mut n: usize = 0;
    while n < total_samples as usize {
        let val: i16 = buff[n / 2];
        end_buffer.push(val);
        end_buffer.push(val);

        n += 2;
    }

    // 파일에 저장하기
    let mut f = File::create(fname).expect("Unalbe to create file");
    let bytes = bincode::serialize(&wh).unwrap();
    let data = bincode::serialize(&end_buffer).unwrap();
    f.write_all(&bytes).expect("Unable to write");
    f.write_all(&data).expect("Append");
}

fn gen(freq: f32, dur: u32, vol: f32) -> Vec<i16> {
    let samplerate: u32 = 44_100;
    let mut phase: f32 = 0.0;
    let phaseincr: f32 = (TWO_PI / (samplerate as f32) * freq).into();
    let mut buffer = Vec::new();
    let total_samples = samplerate * dur;

    for _ in 0..total_samples {
        buffer.push((32767.0 * (phase.sin() * vol)) as i16);
        phase += phaseincr;

        if phase >= TWO_PI {
            phase -= TWO_PI;
        }
    }

    buffer
}

fn main() {
    let buff: Vec<i16> = gen(440.0, 3, 1.0);
    write_wav("test.wav", buff, 3, 44100, 1);
}
