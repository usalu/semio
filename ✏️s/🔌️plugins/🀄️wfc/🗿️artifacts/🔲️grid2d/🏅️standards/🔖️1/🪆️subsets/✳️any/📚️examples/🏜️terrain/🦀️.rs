//! 🏜️ Example `terrain` — the real-world counterpart of `🚰️pipes`: three BITMAP tiles (grass, sand,
//! water) on an 8×8 grid with the classic transition law "water only ever touches sand".
//!
//! Each tile is a 4×4 indexed bitmap over its own two-colour palette, so the preview pane draws it
//! pixel-by-pixel rather than as a flat swatch. Compatibility is symmetric, so a rule is authored
//! for BOTH ordered pairs of every compatible tile pair, in the two canonical directions.

use crate::schema::snapshot::{
    encode_palette_indices, Grid2dSnapshot, WfcAdjacencyRule2d, WfcColor, WfcDirection2d, WfcPinnedCell2d, WfcTile2d, WfcTileMedia2d, WFC_GRID2D_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "terrain";
pub const ICON: &str = "map";
pub const SEED: u64 = 11;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Terrain", "Gelände")
}

/// 🎨️ `(id, label, weight, base colour, speckle colour)` — the two-entry palette each tile's 4×4
/// bitmap indexes into.
const TILES: [(&str, &str, f64, WfcColor, WfcColor); 3] = [
    ("grass", "Grass", 4.0, WfcColor { r: 74, g: 124, b: 63, a: 255 }, WfcColor { r: 96, g: 153, b: 82, a: 255 }),
    ("sand", "Sand", 2.0, WfcColor { r: 214, g: 190, b: 126, a: 255 }, WfcColor { r: 232, g: 212, b: 160, a: 255 }),
    ("water", "Water", 3.0, WfcColor { r: 44, g: 98, b: 156, a: 255 }, WfcColor { r: 66, g: 128, b: 190, a: 255 }),
];

/// 🌊️ Grass may meet grass or sand; sand may meet anything; water may meet water or sand — the one
/// forbidden neighbourhood is grass beside water, which is exactly what makes a beach appear.
fn compatible(left: &str, right: &str) -> bool {
    !matches!((left, right), ("grass", "water") | ("water", "grass"))
}

/// 🧵️ A fixed 4×4 speckle mask — deterministic on purpose: an example that reshuffles its own media
/// between runs is not a fixture.
const SPECKLE: [u8; 16] = [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1];

fn media(base: WfcColor, speckle: WfcColor) -> WfcTileMedia2d {
    WfcTileMedia2d::Bitmap { width: 4, height: 4, palette: vec![base, speckle], pixels: encode_palette_indices(&SPECKLE) }
}

fn tiles() -> Vec<WfcTile2d> {
    let mut tiles: Vec<WfcTile2d> =
        TILES.iter().map(|(id, label, weight, base, speckle)| WfcTile2d { id: (*id).into(), label: Some((*label).into()), weight: *weight, media: media(*base, *speckle) }).collect();
    tiles.sort_by(|left, right| left.id.cmp(&right.id));
    tiles
}

fn rules() -> Vec<WfcAdjacencyRule2d> {
    let mut rules = Vec::new();
    for (a_id, ..) in TILES.iter() {
        for (b_id, ..) in TILES.iter() {
            if !compatible(a_id, b_id) {
                continue;
            }
            for (direction, slug) in [(WfcDirection2d::Right, "right"), (WfcDirection2d::Bottom, "bottom")] {
                rules.push(WfcAdjacencyRule2d { id: format!("rule-{slug}-{a_id}-{b_id}"), tile_a_id: (*a_id).into(), tile_b_id: (*b_id).into(), direction, allowed: true });
            }
        }
    }
    rules.sort_by(|left, right| left.id.cmp(&right.id));
    rules
}

/// 🏜️ The authored problem spec, stated in Rust so the DSL text an example picker loads is a PRINT
/// of this and never a second, drifting authority.
pub fn document() -> Grid2dSnapshot {
    Grid2dSnapshot {
        schema: WFC_GRID2D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        width: 8,
        height: 8,
        cell_width: 24.0,
        cell_height: 24.0,
        periodic_x: true,
        periodic_y: false,
        tiles: tiles(),
        rules: rules(),
        pinned: vec![WfcPinnedCell2d { x: 0, y: 7, tile_id: "water".into() }, WfcPinnedCell2d { x: 7, y: 0, tile_id: "grass".into() }],
        masked: Vec::new(),
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
