//! 🪠️ Example `pipes-3d` — the tiny case: a 3×3×3 pipe network whose only constraint is that an open
//! pipe end must meet another open end, which is the smallest rule set that still forces a real
//! propagation instead of a free fill.
//!
//! 🧭️ Three tiles — `empty`, `pipe-x` (runs along x), `pipe-z` (runs along z). Along x, a `pipe-x`
//! must meet `pipe-x`; along z, a `pipe-z` must meet `pipe-z`; `empty` meets `empty` and the pipe
//! whose axis it does not cut. The grid is periodic on x, so a run of pipe wraps rather than ending
//! in mid-air — the one example that exercises `Boundary::Wrap`.

use crate::schema::snapshot::{Grid3dColor, Grid3dDirection, Grid3dMesh, Grid3dPinnedCell, Grid3dRule, Grid3dSnapshot, Grid3dTile, Grid3dTileMedia, WFC_GRID3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "pipes-3d";
pub const ICON: &str = "layers";
pub const SEED: u64 = 19;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("3D Pipes", "3D-Rohre")
}

pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🪠️pipes-3d/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

pub const TILES: [(&str, &str, f64, [u32; 4]); 3] = [("empty", "Empty", 6.0, [230, 236, 244, 30]), ("pipe-x", "Pipe X", 2.0, [66, 134, 196, 255]), ("pipe-z", "Pipe Z", 2.0, [196, 134, 66, 255])];

/// ⛓️ `(direction, below_or_left, above_or_right)` — the closed allow-list. A pipe continues along
/// its own axis and is only ever met end-to-end; across the other axes it sits beside anything.
pub const ADJACENCIES: [(&str, &str, &str); 11] = [
    ("right", "empty", "empty"),
    ("right", "empty", "pipe-z"),
    ("right", "pipe-z", "empty"),
    ("right", "pipe-z", "pipe-z"),
    ("right", "pipe-x", "pipe-x"),
    ("back", "empty", "empty"),
    ("back", "empty", "pipe-x"),
    ("back", "pipe-x", "empty"),
    ("back", "pipe-x", "pipe-x"),
    ("back", "pipe-z", "pipe-z"),
    ("top", "pipe-z", "pipe-z"),
];

/// ⛓️ Plus the vertical pairs `empty` participates in, kept separate so the table above reads as the
/// pipe law and this as the filler law.
pub const VERTICAL_FILLER: [(&str, &str); 3] = [("empty", "empty"), ("empty", "pipe-x"), ("pipe-x", "empty")];

fn direction(tag: &str) -> Grid3dDirection {
    match tag {
        "back" => Grid3dDirection::Back,
        "top" => Grid3dDirection::Top,
        _ => Grid3dDirection::Right,
    }
}

fn tile(id: &str, label: &str, weight: f64, color: [u32; 4]) -> Grid3dTile {
    Grid3dTile {
        id: id.to_string(),
        label: Some(label.to_string()),
        weight,
        media: Grid3dTileMedia::Mesh { mesh: Grid3dMesh { positions: Vec::new(), indices: Vec::new(), color: Some(Grid3dColor { r: color[0], g: color[1], b: color[2], a: color[3] }) } },
    }
}

/// ⛓️ Every authored rule, sorted by id.
pub fn rules() -> Vec<Grid3dRule> {
    let mut rules: Vec<Grid3dRule> = ADJACENCIES
        .iter()
        .map(|(tag, a, b)| Grid3dRule { id: format!("r-{tag}-{a}-{b}"), tile_a_id: (*a).to_string(), tile_b_id: (*b).to_string(), direction: direction(tag), allowed: true })
        .collect();
    rules.extend(VERTICAL_FILLER.iter().map(|(below, above)| Grid3dRule { id: format!("r-top-{below}-{above}"), tile_a_id: (*below).to_string(), tile_b_id: (*above).to_string(), direction: Grid3dDirection::Top, allowed: true }));
    rules.sort_by(|left, right| left.id.cmp(&right.id));
    rules
}

/// 🪠️ The authored problem spec — the authority its `🗣️.dsl.semio` asset is printed from.
pub fn snapshot() -> Grid3dSnapshot {
    Grid3dSnapshot {
        schema: WFC_GRID3D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        width: 3,
        height: 3,
        depth: 3,
        cell_sizes_x: vec![1.0, 1.0, 1.0],
        cell_sizes_y: vec![1.0, 1.0, 1.0],
        cell_sizes_z: vec![0.5, 1.0, 0.5],
        periodic_x: true,
        periodic_y: false,
        periodic_z: false,
        tiles: TILES.iter().map(|(id, label, weight, color)| tile(id, label, *weight, *color)).collect(),
        rules: rules(),
        pinned: vec![Grid3dPinnedCell { x: 1, y: 1, z: 1, tile_id: "pipe-x".into() }],
        masked: Vec::new(),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
