//! 🗺️ Example `terrain-ring` — the hex ring again, but every tile is an 8×8 palette-indexed BITMAP
//! rather than a vector path.
//!
//! It exists to keep the `Bitmap` media branch honest end to end: the tiles round-trip through the
//! document codecs, the solve treats them like any other pattern, and the preview windows encode them
//! to a real `data:image/png;base64,…` layer instead of drawing a labelled rectangle. Without a
//! bundled example that actually uses `Bitmap`, that whole branch is unexercised by every render test
//! — which is exactly how it stayed a label for one audit cycle.
//!
//! The terrain set is a four-colour transect: `water`, `shore`, `grass`, `rock`. The ring rule set
//! forbids the two ENDS of the transect from touching (`water` beside `rock`), so the ring has to
//! pass through a middle band — the same "propagation really follows the authored edges" property
//! `hex-ring` proves, now over raster tiles.

use crate::schema::snapshot::{Wfc2dColor, Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile, Wfc2dTileMedia, WFC_2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "terrain-ring";
pub const ICON: &str = "map";
pub const SEED: u64 = 88;
/// 🔗 The single adjacency class the ring authors.
pub const RELATION_RING: &str = "ring";
/// 📐 Every tile is this many pixels on a side.
pub const TILE_PIXELS: u32 = 8;
const RADIUS: f64 = 6.0;
const SLOT_SIZE: f64 = 2.0;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Terrain Ring", "Gelände-Ring")
}

/// 🎨 An 8×8 tile whose top `horizon` rows are palette index 0 and the rest index 1 — a two-band
/// terrain patch. Two indices is all a tile needs to prove the palette really is read per pixel.
fn banded_tile(id: &str, label: &str, weight: f64, top: Wfc2dColor, bottom: Wfc2dColor, horizon: u32) -> Wfc2dTile {
    let mut indices = Vec::with_capacity((TILE_PIXELS * TILE_PIXELS) as usize);
    for row in 0..TILE_PIXELS {
        for _ in 0..TILE_PIXELS {
            indices.push(u8::from(row >= horizon));
        }
    }
    Wfc2dTile {
        id: id.into(),
        label: Some(label.into()),
        weight,
        media: Wfc2dTileMedia::Bitmap { width: TILE_PIXELS, height: TILE_PIXELS, palette: vec![top, bottom], pixels: base64_codec::base64_standard_encode(&indices) },
    }
}

fn hex_slot(index: usize) -> Wfc2dSlot {
    let angle = std::f64::consts::FRAC_PI_3 * index as f64;
    let round = |value: f64| (value * 1_000.0).round() / 1_000.0;
    Wfc2dSlot { id: format!("hex-{index}"), x: round(RADIUS * angle.cos()), y: round(RADIUS * angle.sin()), width: SLOT_SIZE, height: SLOT_SIZE, pinned_tile_id: None }
}

fn ring_edge(index: usize) -> Wfc2dSlotEdge {
    let next = (index + 1) % 6;
    Wfc2dSlotEdge { id: format!("edge-{index}-{next}"), from_slot_id: format!("hex-{index}"), to_slot_id: format!("hex-{next}"), relation: RELATION_RING.into() }
}

fn allow(id: &str, a: &str, b: &str) -> Wfc2dRule {
    Wfc2dRule { id: id.into(), tile_a_id: a.into(), tile_b_id: b.into(), relation: None, allowed: true }
}

/// 🗺️ The authored problem spec. Every collection comes out in ascending `id` order by construction.
pub fn document() -> Wfc2dSnapshot {
    Wfc2dSnapshot {
        schema: WFC_2D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        slots: (0..6).map(hex_slot).collect(),
        edges: (0..6).map(ring_edge).collect(),
        tiles: vec![
            banded_tile("grass", "Grass", 2.0, Wfc2dColor { r: 132, g: 204, b: 22, a: 255 }, Wfc2dColor { r: 77, g: 124, b: 15, a: 255 }, 5),
            banded_tile("rock", "Rock", 1.0, Wfc2dColor { r: 168, g: 162, b: 158, a: 255 }, Wfc2dColor { r: 87, g: 83, b: 78, a: 255 }, 2),
            banded_tile("shore", "Shore", 2.0, Wfc2dColor { r: 250, g: 232, b: 176, a: 255 }, Wfc2dColor { r: 214, g: 188, b: 120, a: 255 }, 4),
            banded_tile("water", "Water", 3.0, Wfc2dColor { r: 96, g: 165, b: 250, a: 255 }, Wfc2dColor { r: 30, g: 64, b: 175, a: 255 }, 3),
        ],
        rules: vec![
            allow("rule-grass-grass", "grass", "grass"),
            allow("rule-grass-rock", "grass", "rock"),
            allow("rule-rock-rock", "rock", "rock"),
            allow("rule-shore-grass", "shore", "grass"),
            allow("rule-shore-shore", "shore", "shore"),
            Wfc2dRule { id: "rule-water-rock".into(), tile_a_id: "water".into(), tile_b_id: "rock".into(), relation: Some(RELATION_RING.into()), allowed: false },
            allow("rule-water-shore", "water", "shore"),
            allow("rule-water-water", "water", "water"),
        ],
    }
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), store::ArtifactDsl::print_dsl(&document()), ICON)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
