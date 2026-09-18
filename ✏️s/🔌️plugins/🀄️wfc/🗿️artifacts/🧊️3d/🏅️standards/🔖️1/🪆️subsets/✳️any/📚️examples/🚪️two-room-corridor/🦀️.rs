//! 📚️ Example `two-room-corridor` — the smallest WFC problem that still has a real constraint.
//!
//! Three slots on a line, two `beside` edges, two placeable tiles (`room`, `corridor`). The rule set
//! admits exactly ONE pair — a room next to a corridor — and the rules are an allow-list, so the two
//! same-tile adjacencies are forbidden by never being stated. The only consistent assignments over
//! this topology are therefore the two alternations, `room · corridor · room` and
//! `corridor · room · corridor`; the seed picks one and nothing else is reachable. That forcing is
//! what makes the outcome a committable fixture rather than a seeded sample.

use crate::schema::snapshot::{Color, GraphRule, Slot3d, SlotEdge, Tile, Wfc3dSnapshot, WFC3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "two-room-corridor";
pub const ICON: &str = "map";
pub const SEED: u64 = 7;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Two Rooms And A Corridor", "Zwei Räume und ein Korridor")
}

pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🚪️two-room-corridor/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🧩️ The authored problem spec, stated in Rust so the committed `🗣️.dsl.semio` asset is a PRINT of
/// this and never a second, drifting authority.
pub fn snapshot() -> Wfc3dSnapshot {
    Wfc3dSnapshot {
        schema: WFC3D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        slots: vec![
            Slot3d { id: "corridor".into(), x: 1.0, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: None },
            Slot3d { id: "room-a".into(), x: 0.0, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: None },
            Slot3d { id: "room-b".into(), x: 2.0, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: None },
        ],
        edges: vec![
            SlotEdge { id: "edge-a-corridor".into(), from_slot_id: "room-a".into(), to_slot_id: "corridor".into(), relation: "beside".into() },
            SlotEdge { id: "edge-corridor-b".into(), from_slot_id: "corridor".into(), to_slot_id: "room-b".into(), relation: "beside".into() },
        ],
        tiles: vec![
            Tile { id: "corridor".into(), label: Some("Corridor".into()), weight: 1.0, media: crate::unit_box_media(Some(Color { r: 200, g: 200, b: 210, a: 255 })) },
            Tile { id: "room".into(), label: Some("Room".into()), weight: 2.0, media: crate::unit_box_media(Some(Color { r: 180, g: 140, b: 100, a: 255 })) },
        ],
        rules: vec![GraphRule { id: "rule-room-corridor".into(), tile_a_id: "corridor".into(), tile_b_id: "room".into(), relation: None, allowed: true }],
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
