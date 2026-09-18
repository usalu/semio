//! 🚰️ Example `pipes` — the canonical tiled-model demo: a five-tile VECTOR set on a 6×6 grid whose
//! rotation variants are separate tiles, exactly the way an authored (non-symmetry-expanded) tiled
//! model states them.
//!
//! Each tile declares which of its four sides carries a pipe end; a rule is authored for every
//! ordered pair whose facing connectors agree. Only the `RIGHT` and `BOTTOM` directions are
//! authored: the inference registers each relation with its declared inverse and calls
//! `allow_mirrored`, so a `RIGHT` rule already states the matching `LEFT` one and a second row
//! would be redundant — and `create-rule` would refuse it as a duplicate constraint anyway.

use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d, WfcCell2d, WfcColor, WfcDirection2d, WfcPathSegment, WfcPinnedCell2d, WfcPoint2, WfcTile2d, WfcTileMedia2d, WfcVectorPath, WFC_GRID2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "pipes";
pub const ICON: &str = "network";
pub const SEED: u64 = 7;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Pipes", "Rohre")
}

/// 🔌️ One tile's connector mask, in `[left, right, top, bottom]` order.
const TILES: [(&str, &str, f64, [bool; 4]); 5] = [
    ("empty", "Empty", 3.0, [false, false, false, false]),
    ("pipe-h", "Horizontal Pipe", 2.0, [true, true, false, false]),
    ("pipe-v", "Vertical Pipe", 2.0, [false, false, true, true]),
    ("elbow-ne", "Elbow North-East", 1.0, [false, true, true, false]),
    ("elbow-sw", "Elbow South-West", 1.0, [true, false, false, true]),
];

const INK: WfcColor = WfcColor { r: 56, g: 189, b: 248, a: 255 };

fn point(x: f64, y: f64) -> WfcPoint2 {
    WfcPoint2 { x, y }
}

fn stroke_path(segments: Vec<WfcPathSegment>) -> WfcVectorPath {
    WfcVectorPath { segments, fill: None, stroke: Some(INK), stroke_width: 0.18 }
}

/// 🖊️ The unit-space drawing of one tile — every pipe end runs from the tile centre to the middle
/// of the side it connects through, so two abutting tiles' strokes meet exactly on the seam.
fn media(connectors: [bool; 4]) -> WfcTileMedia2d {
    let centre = point(0.5, 0.5);
    let ends = [(connectors[0], point(0.0, 0.5)), (connectors[1], point(1.0, 0.5)), (connectors[2], point(0.5, 0.0)), (connectors[3], point(0.5, 1.0))];
    let paths = ends
        .into_iter()
        .filter(|(present, _)| *present)
        .map(|(_, end)| stroke_path(vec![WfcPathSegment::MoveTo { to: centre }, WfcPathSegment::LineTo { to: end }]))
        .collect();
    WfcTileMedia2d::Vector { paths }
}

fn tiles() -> Vec<WfcTile2d> {
    let mut tiles: Vec<WfcTile2d> =
        TILES.iter().map(|(id, label, weight, connectors)| WfcTile2d { id: (*id).into(), label: Some((*label).into()), weight: *weight, media: media(*connectors) }).collect();
    tiles.sort_by(|left, right| left.id.cmp(&right.id));
    tiles
}

/// ⛓️ Every ordered pair whose facing connectors agree, in the two canonical directions.
fn rules() -> Vec<WfcAdjacencyRule2d> {
    let mut rules = Vec::new();
    for (a_id, _, _, a) in TILES.iter() {
        for (b_id, _, _, b) in TILES.iter() {
            if a[1] == b[0] {
                rules.push(WfcAdjacencyRule2d { id: format!("rule-right-{a_id}-{b_id}"), tile_a_id: (*a_id).into(), tile_b_id: (*b_id).into(), direction: WfcDirection2d::Right, allowed: true });
            }
            if a[3] == b[2] {
                rules.push(WfcAdjacencyRule2d { id: format!("rule-bottom-{a_id}-{b_id}"), tile_a_id: (*a_id).into(), tile_b_id: (*b_id).into(), direction: WfcDirection2d::Bottom, allowed: true });
            }
        }
    }
    rules.sort_by(|left, right| left.id.cmp(&right.id));
    rules
}

/// 🚰️ The authored problem spec, stated in Rust so the DSL text an example picker loads is a PRINT
/// of this and never a second, drifting authority.
pub fn document() -> Grid2dSnapshot {
    Grid2dSnapshot {
        schema: WFC_GRID2D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        width: 6,
        height: 6,
        cell_width: 32.0,
        cell_height: 32.0,
        periodic_x: false,
        periodic_y: false,
        tiles: tiles(),
        rules: rules(),
        pinned: vec![WfcPinnedCell2d { x: 0, y: 0, tile_id: "elbow-ne".into() }],
        masked: vec![WfcCell2d { x: 5, y: 5 }],
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
