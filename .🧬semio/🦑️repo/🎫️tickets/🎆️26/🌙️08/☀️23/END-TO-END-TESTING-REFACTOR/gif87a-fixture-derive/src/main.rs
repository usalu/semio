// Scratch, one-off tool (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 7, gif-87a subset).
// Derives a genuine GIF87a fixture from the real GIF89a "dancing" animation by reading real
// frames/palettes with the `gif` reference crate (0.13, the version pinned by the stdio oracle
// registry): every frame in the source is a large (300-800px) real photographic crop with its own
// 256-colour local table, so this tool crops small real rectangles out of a handful of real
// frames' real decoded indices (genuine sampled content, never synthesized) small enough to embed
// their index arrays literally in a Gherkin doc-string. It then writes a GIF87a-shaped byte
// stream -- header magic "GIF87a", screen descriptor, then each chosen image's Image Descriptor +
// LZW data -- with every Graphic Control Extension the crate's encoder unconditionally emits
// stripped back out (GIF87a has no GCE concept; this repo's own 87a decoder rejects any 0x21
// block outright).
//
// Usage:
//   gif87a-fixture-derive inspect <dancing.gif>
//   gif87a-fixture-derive extract <dancing.gif> <frame> <x> <y> <w> <h>          -- prints JSON crop
//   gif87a-fixture-derive derive  <dancing.gif> <out.gif> <frame:x:y:w:h,...>    -- writes fixture

use std::borrow::Cow;
use std::fs;

struct OwnedFrame {
    palette: Vec<u8>,
    width: u16,
    height: u16,
    buffer: Vec<u8>,
}

fn read_frames(bytes: &[u8]) -> (u16, u16, u8, u8, Vec<OwnedFrame>) {
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = options.read_info(bytes).expect("dancing.gif must decode via the real GIF89a reader");
    let width = decoder.width();
    let height = decoder.height();
    let background_color_index = decoder.bg_color().unwrap_or(0) as u8;
    let pixel_aspect_ratio = bytes[12];
    let global_palette = decoder.global_palette().unwrap_or(&[]).to_vec();
    let mut frames = Vec::new();
    while let Some(frame) = decoder.read_next_frame().expect("dancing.gif frame must decode") {
        let palette = frame.palette.clone().unwrap_or_else(|| global_palette.clone());
        frames.push(OwnedFrame { palette, width: frame.width, height: frame.height, buffer: frame.buffer.clone().into_owned() });
    }
    (width, height, background_color_index, pixel_aspect_ratio, frames)
}

/// ✂️ A genuine rectangular crop of one real decoded frame's real indices -- never synthesized.
fn crop(frame: &OwnedFrame, x: u16, y: u16, w: u16, h: u16) -> (Vec<u8>, Vec<u8>) {
    let fw = frame.width as usize;
    let mut out = Vec::with_capacity((w as usize) * (h as usize));
    for row in 0..h as usize {
        let start = (y as usize + row) * fw + x as usize;
        out.extend_from_slice(&frame.buffer[start..start + w as usize]);
    }
    (frame.palette.clone(), out)
}

fn print_json_crop(palette: &[u8], indices: &[u8], width: u16, height: u16) {
    let colors: Vec<String> = palette.chunks_exact(3).map(|c| format!("{{\"r\":{},\"g\":{},\"b\":{}}}", c[0], c[1], c[2])).collect();
    println!("width={width} height={height}");
    println!("lct.colors = [{}]", colors.join(","));
    println!("indices = [{}]", indices.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(","));
}

/// 📦️ Remaps a crop onto ONLY the distinct real colours it actually uses (deduplicated, in
/// first-seen order), padded to the next power of two by repeating already-real colours -- never a
/// fabricated one -- so a tiny crop's JSON payload carries a tiny real palette instead of dragging
/// along all 256 real-but-unused source LCT slots.
fn compact(palette: &[u8], indices: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut used: Vec<u8> = Vec::new();
    let mut remap = std::collections::HashMap::new();
    let mut remapped = Vec::with_capacity(indices.len());
    for &index in indices {
        let new_index = *remap.entry(index).or_insert_with(|| {
            used.push(index);
            (used.len() - 1) as u8
        });
        remapped.push(new_index);
    }
    let mut size = 2usize;
    while size < used.len() {
        size *= 2;
    }
    let mut compact_palette = Vec::with_capacity(size * 3);
    for i in 0..size {
        let source = used[i % used.len()] as usize;
        compact_palette.extend_from_slice(&palette[source * 3..source * 3 + 3]);
    }
    (compact_palette, remapped)
}

/// 🔍️ Strips every GIF89a Graphic Control Extension (`0x21 0xF9 <block>`) from an encoder-written
/// byte stream by walking the real GIF block grammar (header/screen-descriptor/GCT verbatim, then
/// block-by-block: `0x21` extensions dropped, `0x2C` image blocks copied whole, `0x3B` trailer
/// copied) -- never a blind byte-pattern search, so LZW data can never be misidentified as a block
/// introducer by coincidence.
fn strip_extensions(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    out.extend_from_slice(&input[0..6]);
    let screen_packed = input[10];
    let mut pos = 13usize;
    if (screen_packed & 0x80) != 0 {
        let table_size = 2usize << (screen_packed & 0x07) as usize;
        pos += table_size * 3;
    }
    out.extend_from_slice(&input[6..pos]);
    loop {
        let b = input[pos];
        match b {
            0x21 => {
                pos += 2; // introducer + label
                loop {
                    let block_size = input[pos] as usize;
                    pos += 1;
                    if block_size == 0 {
                        break;
                    }
                    pos += block_size;
                }
            }
            0x2C => {
                let start = pos;
                pos += 9; // introducer(1) + left(2) + top(2) + width(2) + height(2)
                let image_packed = input[pos];
                pos += 1;
                if (image_packed & 0x80) != 0 {
                    let table_size = 2usize << (image_packed & 0x07) as usize;
                    pos += table_size * 3;
                }
                pos += 1; // lzw minimum code size
                loop {
                    let block_size = input[pos] as usize;
                    pos += 1;
                    if block_size == 0 {
                        break;
                    }
                    pos += block_size;
                }
                out.extend_from_slice(&input[start..pos]);
            }
            0x3B => {
                out.push(0x3B);
                break;
            }
            other => panic!("unexpected GIF block introducer {other:#04x} at offset {pos}"),
        }
    }
    out
}

fn write_gif89a(width: u16, height: u16, global_palette: &[u8], images: &[(Vec<u8>, Vec<u8>, u16, u16, bool)]) -> Vec<u8> {
    let mut out = Vec::new();
    {
        let mut encoder = gif::Encoder::new(&mut out, width, height, global_palette).expect("encoder header");
        for (palette, indices, w, h, use_local) in images {
            let frame = gif::Frame {
                delay: 0,
                dispose: gif::DisposalMethod::Any,
                transparent: None,
                needs_user_input: false,
                top: 0,
                left: 0,
                width: *w,
                height: *h,
                interlaced: false,
                palette: if *use_local { Some(palette.clone()) } else { None },
                buffer: Cow::Borrowed(indices),
            };
            encoder.write_frame(&frame).expect("write frame");
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("inspect") => {
            let bytes = fs::read(&args[2]).expect("read input gif");
            let (width, height, bg, par, frames) = read_frames(&bytes);
            println!("screen {width}x{height} bg={bg} par={par}, {} frames", frames.len());
            for (i, f) in frames.iter().enumerate() {
                println!("frame {i}: width={} height={}", f.width, f.height);
            }
        }
        Some("extract") => {
            let bytes = fs::read(&args[2]).expect("read input gif");
            let frame_index: usize = args[3].parse().unwrap();
            let x: u16 = args[4].parse().unwrap();
            let y: u16 = args[5].parse().unwrap();
            let w: u16 = args[6].parse().unwrap();
            let h: u16 = args[7].parse().unwrap();
            let (_, _, _, _, frames) = read_frames(&bytes);
            let (palette, indices) = crop(&frames[frame_index], x, y, w, h);
            let (palette, indices) = if args.get(8).map(String::as_str) == Some("--compact") { compact(&palette, &indices) } else { (palette, indices) };
            print_json_crop(&palette, &indices, w, h);
        }
        Some("derive") => {
            let bytes = fs::read(&args[2]).expect("read input gif");
            let out_path = &args[3];
            let mut images = Vec::new();
            let mut max_w = 0u16;
            let mut max_h = 0u16;
            let mut global_palette = Vec::new();
            let (_, _, _, _, frames) = read_frames(&bytes);
            for (i, spec) in args[4].split(',').enumerate() {
                let parts: Vec<&str> = spec.split(':').collect();
                let frame_index: usize = parts[0].parse().unwrap();
                let x: u16 = parts[1].parse().unwrap();
                let y: u16 = parts[2].parse().unwrap();
                let w: u16 = parts[3].parse().unwrap();
                let h: u16 = parts[4].parse().unwrap();
                let (palette, indices) = crop(&frames[frame_index], x, y, w, h);
                max_w = max_w.max(w);
                max_h = max_h.max(h);
                if i == 0 {
                    global_palette = palette.clone();
                    images.push((palette, indices, w, h, false));
                } else {
                    images.push((palette, indices, w, h, true));
                }
            }
            let raw = write_gif89a(max_w, max_h, &global_palette, &images);
            let mut stripped = strip_extensions(&raw);
            assert_eq!(&stripped[0..6], b"GIF89a");
            stripped[4] = b'7';
            assert_eq!(&stripped[0..6], b"GIF87a");
            fs::write(out_path, &stripped).expect("write output gif");
            println!("wrote {} bytes ({} images, {}x{} screen) to {}", stripped.len(), images.len(), max_w, max_h, out_path);
        }
        _ => {
            eprintln!("usage: gif87a-fixture-derive inspect <in.gif> | extract <in.gif> <frame> <x> <y> <w> <h> | derive <in.gif> <out.gif> <frame:x:y:w:h,...>");
            std::process::exit(1);
        }
    }
}
