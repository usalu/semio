//! One-off derivation of the real BMP v3 mutation fixture. Reads the real architectural floor plan
//! PNG (8-bit palette, 2334x2560) with the independent `png` decoder — preserving its genuine index
//! buffer and palette table rather than resolving to RGB — and re-encodes it as an 8-bit indexed BMP
//! with the `image` 0.25 reference encoder's palette-aware BMP writer, so the committed fixture
//! genuinely exercises BMP's palette path instead of being downgraded to 24-bit RGB.

use image::codecs::bmp::BmpEncoder;
use image::ExtendedColorType;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let src = std::env::args().nth(1).expect("usage: bmp-fixture-gen <source.png> <dest.bmp>");
    let dst = std::env::args().nth(2).expect("usage: bmp-fixture-gen <source.png> <dest.bmp>");

    let decoder = png::Decoder::new(BufReader::new(File::open(&src).expect("open source png")));
    let mut reader = decoder.read_info().expect("read png info");
    let mut buffer = vec![0u8; reader.output_buffer_size().unwrap_or(0)];
    let frame = reader.next_frame(&mut buffer).expect("decode png frame");
    let info = reader.info();

    assert_eq!(frame.color_type, png::ColorType::Indexed, "source png must be 8-bit palette to exercise BMP's palette path");
    assert_eq!(frame.bit_depth, png::BitDepth::Eight, "source png must be 8-bit indexed");

    let indices = &buffer[..frame.buffer_size()];
    assert_eq!(indices.len(), (frame.width * frame.height) as usize, "indexed 8-bit png must carry one byte per pixel");

    let palette_rgb = info.palette.as_deref().expect("indexed png must carry a PLTE chunk");
    let palette: Vec<[u8; 3]> = palette_rgb.chunks_exact(3).map(|rgb| [rgb[0], rgb[1], rgb[2]]).collect();
    eprintln!("source: {}x{} indexed, {} palette entries", frame.width, frame.height, palette.len());

    let mut out = File::create(&dst).expect("create dest bmp");
    let mut encoder = BmpEncoder::new(&mut out);
    encoder.encode_with_palette(indices, frame.width, frame.height, ExtendedColorType::L8, Some(&palette)).expect("encode indexed bmp");

    let written = std::fs::metadata(&dst).expect("stat dest").len();
    eprintln!("wrote {} bytes to {}", written, dst);
}
