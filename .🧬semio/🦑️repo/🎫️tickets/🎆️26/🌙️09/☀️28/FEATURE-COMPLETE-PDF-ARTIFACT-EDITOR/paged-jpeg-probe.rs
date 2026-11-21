//! 🔬️ Replays the staged JPEG cursor against a discontiguous source and independently generated fixtures.
#![allow(dead_code)]

use std::{cell::Cell, collections::HashMap, fmt::Write, fs, io::Read, path::Path};

#[path = "live-hash.rs"]
mod semio_framework_hash;
#[path = "live-jpeg-transforms.rs"]
mod transforms;
#[path = "jpg-baseline-proof.rs"]
mod proof;
#[path = "jpg-entropy.rs"]
mod entropy;
#[path = "source-trait.rs"]
mod source_trait;

use entropy::*;
use proof::*;
use source_trait::JpgByteSource;
use transforms::{idct_8x8, ycbcr_to_rgb, JpgError, ZIGZAG_TO_NATURAL};

struct ArtifactRef { artifact_id: String, dialect: ArtifactDialect }
struct ArtifactDialect { artifact_kind: String, standard: String, subset: String }

include!("identity-core.rs");

struct Discontiguous {
    pages: Vec<Box<[u8]>>,
    length: usize,
    visits: Cell<usize>,
}

impl Discontiguous {
    const PAGE: usize = 4096;

    fn read(mut file: fs::File) -> std::io::Result<Self> {
        let mut pages = Vec::new();
        let mut length = 0;
        loop {
            let mut page = Vec::with_capacity(Self::PAGE);
            (&mut file).take(Self::PAGE as u64).read_to_end(&mut page)?;
            if page.is_empty() { break; }
            length += page.len();
            pages.push(page.into_boxed_slice());
        }
        Ok(Self { pages, length, visits: Cell::new(0) })
    }
}

impl JpgByteSource for Discontiguous {
    fn len(&self) -> usize { self.length }

    fn byte(&self, index: usize) -> Option<u8> {
        self.visits.set(self.visits.get() + 1);
        self.pages.get(index / Self::PAGE)?.get(index % Self::PAGE).copied()
    }
}

const CROP_WIDTH: usize = 11;
const CROP_HEIGHT: usize = 10;
const CROP_X: usize = 13;
const CROP_Y: usize = 12;

fn examine(input: &Discontiguous) -> Result<String, JpgError> {
    let mut cursor = BaselineJpgCursor::new(input.len());
    let mut planes = [[[0f64; CROP_WIDTH]; CROP_HEIGHT]; 3];
    let mut max_reads = 0;
    while cursor.complete().is_none() {
        input.visits.set(0);
        cursor.step(input, &mut |block| {
            let mut matched = [(0usize, 0usize, 0usize); CROP_WIDTH * CROP_HEIGHT];
            let mut count = 0;
            for y in 0..CROP_HEIGHT {
                let sy = (y + CROP_Y) * block.sampling_y / block.maximum_sampling_y;
                if sy / 8 != block.block_y { continue; }
                for x in 0..CROP_WIDTH {
                    let sx = (x + CROP_X) * block.sampling_x / block.maximum_sampling_x;
                    if sx / 8 != block.block_x { continue; }
                    matched[count] = (x, y, sy % 8 * 8 + sx % 8);
                    count += 1;
                }
            }
            if count == 0 { return; }
            let mut coefficients = [0.0; 64];
            for (z, &natural) in ZIGZAG_TO_NATURAL.iter().enumerate() {
                coefficients[natural] = f64::from(block.zigzag[z]) * f64::from(block.quantization[z]);
            }
            let spatial = idct_8x8(&coefficients);
            for &(x, y, index) in &matched[..count] {
                planes[block.component][y][x] = spatial[index] + 128.0;
            }
        })?;
        max_reads = max_reads.max(input.visits.get());
    }
    assert!(max_reads <= 4096);

    let metadata = cursor.complete().unwrap();
    let mut hash = PdfDctResourceIdentity::new(metadata.color_transform).unwrap();
    for page in &input.pages { hash.append(page); }
    assert!(hash.complete(input.len()));
    let transform = metadata.color_transform.map_or(String::from("None"), |num| num.to_string());
    let mut output = format!("{} {} {} {}\n{}\n", metadata.width, metadata.height, metadata.components,
        transform, hash.reference().artifact_id);
    let normalize = |sample: f64| sample.round().clamp(0.0, 255.0) as u8;
    for y in 0..CROP_HEIGHT {
        for x in 0..CROP_WIDTH {
            let [first, second, third] = planes.each_ref().map(|plane| plane[y][x]);
            let (r, g, b) = match metadata.color_transform {
                None => (normalize(first), normalize(first), normalize(first)),
                Some(0) => (normalize(first), normalize(second), normalize(third)),
                Some(1) => ycbcr_to_rgb(first, second, third),
                _ => unreachable!(),
            };
            write!(output, "{r:02x}{g:02x}{b:02x}").unwrap();
        }
    }
    writeln!(output, "\n{max_reads}").unwrap();
    Ok(output)
}

fn main() -> std::io::Result<()> {
    let dir = std::env::args_os().nth(1).unwrap();
    let dir = Path::new(&dir);
    for entry in dir.read_dir()? {
        let path = entry?.path();
        if path.extension().is_none_or(|ext| ext != "jpg") { continue; }
        let input = Discontiguous::read(fs::File::open(&path)?)?;
        let output = examine(&input).unwrap_or_else(|error| match error {
            JpgError::Malformed(message) | JpgError::Unsupported(message) => message,
        });
        fs::write(path.with_extension("actual"), output)?;
    }
    Ok(())
}
