//! 🧱️ Example `blocks` — the real-world case: an architectural block set on a genuinely NON-UNIFORM
//! 4×3×4 grid (wide middle bays on x, a tall ground storey on y, a thick top layer on z).
//!
//! 🧭️ Four tiles — `air`, `floor`, `roof`, `wall` — with a closed allow-list: horizontally anything
//! may stand beside anything, VERTICALLY the stack is forced (`floor` carries `wall` or `air`, `wall`
//! carries `wall` or `roof`, `roof` carries only `air`, `air` carries only `air`). One pinned floor
//! cell anchors the ground, one masked cell carves a notch out of the box — so the example exercises
//! the pin lane, the mask lane and the non-uniform geometry at once, and is still satisfiable.

use crate::schema::snapshot::{Grid3dCell, Grid3dColor, Grid3dDirection, Grid3dMesh, Grid3dPinnedCell, Grid3dRule, Grid3dSnapshot, Grid3dTile, Grid3dTileMedia, WFC_GRID3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "blocks";
pub const ICON: &str = "building";
pub const SEED: u64 = 7;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Building Blocks", "Bauklötze")
}

pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🧱️blocks/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🧱️ The four tiles, in canonical id order.
pub const TILES: [(&str, &str, f64, [u32; 4]); 4] =
    [("air", "Air", 4.0, [200, 214, 232, 40]), ("floor", "Floor", 1.0, [120, 120, 126, 255]), ("roof", "Roof", 1.5, [172, 84, 62, 255]), ("wall", "Wall", 3.0, [212, 204, 188, 255])];

/// ⛓️ The vertical stacking law, `(below, above)` — every other vertical pair is forbidden simply by
/// not appearing here, because the rule set is a closed allow-list.
pub const STACK: [(&str, &str); 6] = [("air", "air"), ("floor", "air"), ("floor", "wall"), ("roof", "air"), ("wall", "roof"), ("wall", "wall")];

fn tile(id: &str, label: &str, weight: f64, color: [u32; 4]) -> Grid3dTile {
    Grid3dTile {
        id: id.to_string(),
        label: Some(label.to_string()),
        weight,
        media: Grid3dTileMedia::Mesh { mesh: Grid3dMesh { positions: Vec::new(), indices: Vec::new(), color: Some(Grid3dColor { r: color[0], g: color[1], b: color[2], a: color[3] }) } },
    }
}

/// ⛓️ Every authored rule, sorted by id — the canonical order every collection mutation inserts at.
pub fn rules() -> Vec<Grid3dRule> {
    let mut rules = Vec::new();
    for (direction, tag) in [(Grid3dDirection::Right, "right"), (Grid3dDirection::Back, "back")] {
        for (a, ..) in TILES {
            for (b, ..) in TILES {
                rules.push(Grid3dRule { id: format!("r-{tag}-{a}-{b}"), tile_a_id: a.to_string(), tile_b_id: b.to_string(), direction, allowed: true });
            }
        }
    }
    for (below, above) in STACK {
        rules.push(Grid3dRule { id: format!("r-top-{below}-{above}"), tile_a_id: below.to_string(), tile_b_id: above.to_string(), direction: Grid3dDirection::Top, allowed: true });
    }
    rules.sort_by(|left, right| left.id.cmp(&right.id));
    rules
}

/// 🧱️ The authored problem spec, stated in Rust so the committed `🗣️.dsl.semio` asset is a PRINT of
/// this and never a second, drifting authority.
pub fn snapshot() -> Grid3dSnapshot {
    Grid3dSnapshot {
        schema: WFC_GRID3D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        width: 4,
        height: 3,
        depth: 4,
        cell_sizes_x: vec![1.0, 2.0, 2.0, 1.0],
        cell_sizes_y: vec![3.0, 1.5, 1.5],
        cell_sizes_z: vec![1.0, 1.0, 1.0, 2.0],
        periodic_x: false,
        periodic_y: false,
        periodic_z: false,
        tiles: TILES.iter().map(|(id, label, weight, color)| tile(id, label, *weight, *color)).collect(),
        rules: rules(),
        pinned: vec![Grid3dPinnedCell { x: 0, y: 0, z: 0, tile_id: "floor".into() }],
        masked: vec![Grid3dCell { x: 3, y: 2, z: 3 }],
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
