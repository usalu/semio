//! 🚪️ Example `two-room-corridor` — the smallest WFC 2D problem that still has a real constraint.
//!
//! Three slots on a line, two adjacency edges, two tiles (`room`, `corridor`) drawn as flat vector
//! rectangles. The rule set forbids both same-tile adjacencies, so the ONLY consistent assignment
//! over this topology is `room · corridor · room` — the example exists to make that forcing visible:
//! the solver has no freedom left, which is what makes its outcome a committable fixture rather than
//! a seeded sample.

use crate::schema::snapshot::{Wfc2dColor, Wfc2dPathSegment, Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile, Wfc2dTileMedia, Wfc2dVectorPath, WFC_2D_DEFAULT_RELATION, WFC_2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "two-room-corridor";
pub const ICON: &str = "workflow";
pub const SEED: u64 = 7;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Two Rooms And A Corridor", "Zwei Räume und ein Korridor")
}

/// 🎨 A full-bleed filled square in tile space — the simplest honest vector tile.
pub(crate) fn filled_square(color: Wfc2dColor) -> Wfc2dTileMedia {
    Wfc2dTileMedia::Vector {
        paths: vec![Wfc2dVectorPath {
            segments: vec![
                Wfc2dPathSegment::Move { to: [0.05, 0.05] },
                Wfc2dPathSegment::Line { to: [0.95, 0.05] },
                Wfc2dPathSegment::Line { to: [0.95, 0.95] },
                Wfc2dPathSegment::Line { to: [0.05, 0.95] },
                Wfc2dPathSegment::Close,
            ],
            fill: Some(color),
            stroke: None,
            stroke_width: 0.0,
        }],
    }
}

fn slot(id: &str, x: f64) -> Wfc2dSlot {
    Wfc2dSlot { id: id.into(), x, y: 0.0, width: 2.0, height: 2.0, pinned_tile_id: None }
}

fn edge(id: &str, from: &str, to: &str) -> Wfc2dSlotEdge {
    Wfc2dSlotEdge { id: id.into(), from_slot_id: from.into(), to_slot_id: to.into(), relation: WFC_2D_DEFAULT_RELATION.into() }
}

/// 🚪️ The authored problem spec, stated in Rust so the printed document is never a second,
/// drifting authority. Every collection is in canonical ascending `id` order.
pub fn document() -> Wfc2dSnapshot {
    Wfc2dSnapshot {
        schema: WFC_2D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        slots: vec![slot("corridor", 2.0), slot("room-a", 0.0), slot("room-b", 4.0)],
        edges: vec![edge("edge-a-corridor", "room-a", "corridor"), edge("edge-corridor-b", "corridor", "room-b")],
        tiles: vec![
            Wfc2dTile { id: "corridor".into(), label: Some("Corridor".into()), weight: 1.0, media: filled_square(Wfc2dColor { r: 148, g: 163, b: 184, a: 255 }) },
            Wfc2dTile { id: "room".into(), label: Some("Room".into()), weight: 2.0, media: filled_square(Wfc2dColor { r: 96, g: 165, b: 250, a: 255 }) },
        ],
        rules: vec![
            Wfc2dRule { id: "rule-corridor-corridor".into(), tile_a_id: "corridor".into(), tile_b_id: "corridor".into(), relation: None, allowed: false },
            Wfc2dRule { id: "rule-room-corridor".into(), tile_a_id: "room".into(), tile_b_id: "corridor".into(), relation: None, allowed: true },
            Wfc2dRule { id: "rule-room-room".into(), tile_a_id: "room".into(), tile_b_id: "room".into(), relation: None, allowed: false },
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
