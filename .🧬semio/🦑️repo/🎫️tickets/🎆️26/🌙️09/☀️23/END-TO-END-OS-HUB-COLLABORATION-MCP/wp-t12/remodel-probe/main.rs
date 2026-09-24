//! ⏱️ Ticket-local probe (T12): runs the two remodel PNG worker-ceiling fixtures through the production
//! `BoundedStillDecoder` and records every step's wall time with its decoder phase, so the per-step cost is measured
//! (serially, per phase) instead of inferred from a pass/fail wall-clock law.
use semio_s_artifact_remodel_remodeling::editor::remodeling::engine::images::{BoundedDecodeProgress, BoundedStillDecoder, CompressedChunkRope};

fn rope(bytes: &[u8]) -> CompressedChunkRope {
    let mut rope = CompressedChunkRope::default();
    for chunk in bytes.chunks(3_072) {
        rope.push(std::sync::Arc::<[u8]>::from(chunk), 1_114_112).expect("bounded fixture");
    }
    rope
}

fn encode(width: u32, height: u32, pixels: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut encoded, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.write_header().expect("header").write_image_data(pixels).expect("data");
    }
    encoded
}

fn probe(label: &str, bytes: &[u8]) {
    let mut decoder = BoundedStillDecoder::new("image/png", rope(bytes));
    let mut steps: Vec<(u128, usize)> = Vec::new();
    loop {
        let started = std::time::Instant::now();
        let progress = decoder.advance();
        steps.push((started.elapsed().as_micros(), steps.len()));
        match progress {
            BoundedDecodeProgress::Working => {}
            BoundedDecodeProgress::Complete(_) => break,
            BoundedDecodeProgress::Failed(error) => panic!("{label}: {error:?}"),
        }
    }
    let total: u128 = steps.iter().map(|(us, _)| us).sum();
    let mut worst = steps.clone();
    worst.sort_by(|a, b| b.0.cmp(&a.0));
    let over = steps.iter().filter(|(us, _)| *us >= 8_000).count();
    println!("{label}: {} steps, total {total} us, over 8 ms: {over}, worst (us@step): {:?}", steps.len(), &worst[..worst.len().min(12)]);
}

fn main() {
    let (w, h) = (512u32, 512u32);
    probe("uniform-512x512", &encode(w, h, &vec![127u8; (w * h * 4) as usize]));
    let (w, h) = (4_096u32, 64u32);
    let mut pixels = vec![0u8; (w * h * 4) as usize];
    for index in 0..(w * h) as usize {
        let value = ((index * 131) ^ (index / w as usize * 197)) as u8;
        pixels[index * 4..index * 4 + 4].copy_from_slice(&[value, value.rotate_left(3), !value, 255]);
    }
    probe("max-row-4096x64", &encode(w, h, &pixels));
}
