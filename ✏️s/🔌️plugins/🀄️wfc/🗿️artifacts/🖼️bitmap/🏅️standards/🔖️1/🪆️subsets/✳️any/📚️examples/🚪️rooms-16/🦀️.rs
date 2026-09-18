//! 🚪️ Example `rooms-16` — the smallest sample that still teaches a real overlapping model: a
//! 16 × 16, three-colour plan of five-cell rooms separated by walls, with a door punched through
//! every wall segment's midpoint.
//!
//! The sample is COMPUTED, not transcribed: a hand-typed 256-byte index buffer is unreadable and
//! undiffable, while a closed form states exactly the regularity the model is supposed to learn —
//! walls on the five-grid, doors at the midpoints, floor everywhere else. The committed
//! `🗣️.dsl.semio` asset is the PRINT of this builder and never a second authority.

use crate::schema::snapshot::{encode_base64, BitmapColor, BitmapInput, BitmapOutputSpec, BitmapOverlappingModel, BitmapSnapshot, WFC_BITMAP_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "rooms-16";
pub const ICON: &str = "layout-grid";
pub const SEED: u64 = 7;
pub const EDGE: u32 = 16;
/// 🚪️ Room pitch: walls sit on every fifth column and row, so a room interior is four cells wide.
const PITCH: u32 = 5;

/// 🎨️ `0` wall, `1` floor, `2` door — the index vocabulary the whole example is written in.
pub const WALL: u8 = 0;
pub const FLOOR: u8 = 1;
pub const DOOR: u8 = 2;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Rooms 16", "Räume 16")
}

pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🚪️rooms-16/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🚪️ The sample's palette indices, row-major.
pub fn indices() -> Vec<u8> {
    let mut buffer = vec![FLOOR; (EDGE as usize) * (EDGE as usize)];
    for y in 0..EDGE {
        for x in 0..EDGE {
            let on_wall = x % PITCH == 0 || y % PITCH == 0;
            let door = (x % PITCH == 0 && y % PITCH == PITCH / 2) || (y % PITCH == 0 && x % PITCH == PITCH / 2);
            buffer[(y * EDGE + x) as usize] = if door { DOOR } else if on_wall { WALL } else { FLOOR };
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
            palette: vec![BitmapColor::opaque(38, 38, 46), BitmapColor::opaque(226, 222, 210), BitmapColor::opaque(198, 132, 72)],
            pixels: encode_base64(&indices()),
        },
        output: BitmapOutputSpec { width: 24, height: 24, periodic: true },
        model: BitmapOverlappingModel { pattern_size: 3, symmetry: 8, periodic_input: true, ground: None },
        pinned: Vec::new(),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
