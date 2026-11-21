//! 🔬️ Drives the staged JPEG preview with independent source images and split sample slots.
#![allow(dead_code)]

use std::{cell::Cell, collections::HashMap, fs, io::{Read, Write}, path::Path};

#[path = "live-jpeg-transforms.rs"]
mod transforms;
#[path = "jpg-baseline-proof.rs"]
mod proof;
#[path = "jpg-entropy.rs"]
mod entropy;
#[path = "source-trait.rs"]
mod source_trait;
#[path = "jpg-preview.rs"]
mod preview;

use entropy::*;
use proof::*;
use source_trait::JpgByteSource;
use transforms::{idct_8x8, ycbcr_to_rgb, JpgError, ZIGZAG_TO_NATURAL};

const PAGE: usize = 4096;

struct Source {
    pages: Vec<Box<[u8]>>,
    bytes: usize,
    accesses: Cell<usize>,
}

impl Source {
    fn read(path: &Path) -> std::io::Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut source = Self { pages: Vec::new(), bytes: 0, accesses: Cell::new(0) };
        loop {
            let mut page = Vec::with_capacity(PAGE);
            (&mut file).take(PAGE as u64).read_to_end(&mut page)?;
            if page.is_empty() { return Ok(source); }
            source.bytes += page.len();
            source.pages.push(page.into_boxed_slice());
        }
    }
}

impl JpgByteSource for Source {
    fn len(&self) -> usize { self.bytes }
    fn byte(&self, index: usize) -> Option<u8> {
        self.accesses.set(self.accesses.get() + 1);
        self.pages.get(index / PAGE)?.get(index % PAGE).copied()
    }
}

fn decode(source: &Source, size: preview::JpgPreviewSize, path: &Path, limit: usize) -> Result<(), JpgError> {
    let mut cursor = preview::BaselineJpgPreview::new(source.len(), size)?;
    let [width, height] = size.display;
    let channels = usize::from(size.channels);
    let slots = width * height * channels;
    let mut pages: Vec<_> = (0..slots.div_ceil(PAGE)).map(|_| Box::new([0; PAGE])).collect::<Vec<Box<[u8; PAGE]>>>();
    let mut written: Vec<_> = (0..slots.div_ceil(PAGE)).map(|_| Box::new([false; PAGE])).collect::<Vec<Box<[bool; PAGE]>>>();
    let (mut steps, mut writes, mut peak_writes, mut peak_reads) = (0, 0, 0, 0);
    let metadata = loop {
        if steps == limit {
            assert!(!path.exists());
            println!("[DEBUG] Cancelled after {steps} unpublished preview turns");
            return Ok(());
        }
        let mut block = 0;
        source.accesses.set(0);
        let next = cursor.step(source, &mut |x, y, c, v| {
            assert!(x < width && y < height && c < channels);
            let offset = (y * width + x) * channels + c;
            let (page, slot) = (offset / PAGE, offset % PAGE);
            assert!(!written[page][slot], "duplicate preview slot {offset}");
            written[page][slot] = true;
            pages[page][slot] = v;
            block += 1;
        })?;
        steps += 1;
        writes += block;
        peak_writes = peak_writes.max(block);
        peak_reads = peak_reads.max(source.accesses.get());
        if let Some(metadata) = next { break metadata; }
    };
    assert_eq!(writes, slots);
    assert!(peak_writes <= 1024 && peak_reads <= PAGE);
    let mut out = std::io::BufWriter::new(fs::File::create(path).unwrap());
    let sample = |index: usize| pages[index / PAGE][index % PAGE];
    for px in 0..width * height {
        let first = sample(px * channels);
        let rgb = match metadata.color_transform {
            None => [first; 3],
            Some(0) => [first, sample(px * channels + 1), sample(px * channels + 2)],
            Some(1) => {
                let (r, g, b) = ycbcr_to_rgb(f64::from(first), f64::from(sample(px * channels + 1)), f64::from(sample(px * channels + 2)));
                [r, g, b]
            }
            _ => unreachable!(),
        };
        out.write_all(&rgb).unwrap();
    }
    println!("[DEBUG] {} input bytes, {} sampled components, {} physical sample pages, {} turns, peak {} reads and {} writes/turn, transform {:?}",
        source.len(), writes, pages.len(), steps, peak_reads, peak_writes, metadata.color_transform);
    Ok(())
}

fn main() {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    let source = Source::read(Path::new(&a[0])).unwrap();
    let value = |index: usize| a[index].parse::<usize>().unwrap();
    let size = preview::JpgPreviewSize { source: [value(2), value(3)], display: [value(4), value(5)], channels: value(6) as u8 };
    if let Err(error) = decode(&source, size, Path::new(&a[1]), a.get(7).map_or(usize::MAX, |_| value(7))) {
        match error {
            JpgError::Malformed(message) | JpgError::Unsupported(message) => eprintln!("[DEBUG] {message}"),
        }
        std::process::exit(2);
    }
}
