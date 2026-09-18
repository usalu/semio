//! 🔷️ Example `hex-ring` — six slots on a hexagon, wired into a CYCLE of six edges.
//!
//! The point of this example is the shape: nothing about it fits a rectangular grid, which is the
//! whole reason the `wfc2d` artifact exists beside `grid2d`. Slot centres sit on a regular hexagon,
//! and the tiles alternate `cap`/`link` because `cap` may not touch `cap` — an odd cycle of six
//! nodes with a two-colour constraint is satisfiable, and a five-node ring would not be, which makes
//! this the smallest honest test that the propagation really is following the authored edges.

use crate::schema::snapshot::{Wfc2dColor, Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile, WFC_2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "hex-ring";
pub const ICON: &str = "hexagon";
pub const SEED: u64 = 2026;
/// 🔗 The single adjacency class the ring authors.
pub const RELATION_RING: &str = "ring";
/// 📐 Distance from the ring centre to a slot centre.
const RADIUS: f64 = 6.0;
const SLOT_SIZE: f64 = 2.0;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Hexagonal Ring", "Sechseckiger Ring")
}

/// 📐 The `index`-th vertex of a regular hexagon, rounded to 3 decimals so the printed document is
/// stable across platforms rather than carrying full float noise.
fn hex_slot(index: usize) -> Wfc2dSlot {
    let angle = std::f64::consts::FRAC_PI_3 * index as f64;
    let round = |value: f64| (value * 1_000.0).round() / 1_000.0;
    Wfc2dSlot { id: format!("hex-{index}"), x: round(RADIUS * angle.cos()), y: round(RADIUS * angle.sin()), width: SLOT_SIZE, height: SLOT_SIZE, pinned_tile_id: None }
}

fn ring_edge(index: usize) -> Wfc2dSlotEdge {
    let next = (index + 1) % 6;
    Wfc2dSlotEdge { id: format!("edge-{index}-{next}"), from_slot_id: format!("hex-{index}"), to_slot_id: format!("hex-{next}"), relation: RELATION_RING.into() }
}

/// 🔷️ The authored problem spec. Both collections come out in ascending `id` order by construction.
pub fn document() -> Wfc2dSnapshot {
    Wfc2dSnapshot {
        schema: WFC_2D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        slots: (0..6).map(hex_slot).collect(),
        edges: (0..6).map(ring_edge).collect(),
        tiles: vec![
            Wfc2dTile { id: "cap".into(), label: Some("Cap".into()), weight: 1.0, media: super::two_room_corridor::filled_square(Wfc2dColor { r: 250, g: 204, b: 21, a: 255 }) },
            Wfc2dTile { id: "link".into(), label: Some("Link".into()), weight: 2.0, media: super::two_room_corridor::filled_square(Wfc2dColor { r: 52, g: 211, b: 153, a: 255 }) },
        ],
        rules: vec![
            Wfc2dRule { id: "rule-cap-cap".into(), tile_a_id: "cap".into(), tile_b_id: "cap".into(), relation: Some(RELATION_RING.into()), allowed: false },
            Wfc2dRule { id: "rule-cap-link".into(), tile_a_id: "cap".into(), tile_b_id: "link".into(), relation: None, allowed: true },
            Wfc2dRule { id: "rule-link-link".into(), tile_a_id: "link".into(), tile_b_id: "link".into(), relation: None, allowed: true },
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
