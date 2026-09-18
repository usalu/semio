//! 🌸️ Example `flowers-24` — a 24 × 24, four-colour meadow: a ground band, stems rising from it on
//! a six-cell pitch, and a three-cell petal crown on each. It exercises the parts `rooms-16` does
//! not: a GROUND colour the solve pins across the output's bottom row, a non-periodic input so
//! patterns are only taken from fully in-bounds windows, and a narrower symmetry group, because a
//! flower that is upside down is not a flower.

use crate::schema::snapshot::{encode_base64, BitmapColor, BitmapInput, BitmapOutputSpec, BitmapOverlappingModel, BitmapPinnedPixel, BitmapSnapshot, WFC_BITMAP_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "flowers-24";
pub const ICON: &str = "sparkles";
pub const SEED: u64 = 19;
pub const EDGE: u32 = 24;
/// 🌸️ Stem pitch and the row band the meadow occupies.
const PITCH: u32 = 6;
const GROUND_ROWS: u32 = 3;

/// 🎨️ `0` sky, `1` ground, `2` stem, `3` petal.
pub const SKY: u8 = 0;
pub const GROUND: u8 = 1;
pub const STEM: u8 = 2;
pub const PETAL: u8 = 3;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Flowers 24", "Blumen 24")
}

pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🌸️flowers-24/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🌸️ The sample's palette indices, row-major.
pub fn indices() -> Vec<u8> {
    let mut buffer = vec![SKY; (EDGE as usize) * (EDGE as usize)];
    let ground_top = EDGE - GROUND_ROWS;
    for y in ground_top..EDGE {
        for x in 0..EDGE {
            buffer[(y * EDGE + x) as usize] = GROUND;
        }
    }
    for x in (PITCH / 2..EDGE).step_by(PITCH as usize) {
        for y in (ground_top - 6)..ground_top {
            buffer[(y * EDGE + x) as usize] = STEM;
        }
        let crown = ground_top - 7;
        for offset in 0..3u32 {
            let petal_x = x + offset - 1;
            if petal_x < EDGE {
                buffer[(crown * EDGE + petal_x) as usize] = PETAL;
            }
        }
        if crown > 0 {
            buffer[((crown - 1) * EDGE + x) as usize] = PETAL;
        }
    }
    buffer
}

/// 🧩️ The authored problem spec, stated in Rust so the committed `🗣️.dsl.semio` asset is a PRINT of
/// this and never a second, drifting authority.
pub fn snapshot() -> BitmapSnapshot {
    BitmapSnapshot {
        schema: WFC_BITMAP_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        input: BitmapInput {
            width: EDGE,
            height: EDGE,
            palette: vec![BitmapColor::opaque(148, 198, 232), BitmapColor::opaque(96, 120, 64), BitmapColor::opaque(62, 142, 78), BitmapColor::opaque(232, 108, 142)],
            pixels: encode_base64(&indices()),
        },
        output: BitmapOutputSpec { width: 32, height: 24, periodic: false },
        model: BitmapOverlappingModel { pattern_size: 3, symmetry: 2, periodic_input: false, ground: Some(u32::from(GROUND)) },
        pinned: vec![BitmapPinnedPixel { x: 0, y: 0, color: u32::from(SKY) }],
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
