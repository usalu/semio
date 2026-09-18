//! 📚️ Example `wall-roof-facade-strip` — a two-storey facade strip, the smallest problem in which
//! the topology is a CYCLE rather than a path.
//!
//! Four slots in a 2×2 lattice (two ground bays, two roof bays above them), four adjacency edges
//! split across TWO relations (`beside` along a row, `above` up a stack). Every pair is admitted for
//! every relation, and one DENY then takes `roof` beside `roof` back — a deny always beats an admit,
//! which is the law this example exists to pin. The top-right bay is pinned to `roof`, so the
//! top-left bay is forced to `wall` while the ground row stays free: satisfiable, and still forcing a
//! choice. Together with `two-room-corridor` (a forced acyclic path) this covers the two topologies
//! the engine's propagation treats differently.

use crate::schema::snapshot::{Color, GraphRule, Slot3d, SlotEdge, Tile, Wfc3dSnapshot, WFC3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "wall-roof-facade-strip";
pub const ICON: &str = "layers";
pub const SEED: u64 = 42;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Wall And Roof Facade Strip", "Wand-Dach-Fassadenstreifen")
}

pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🧱️wall-roof-facade-strip/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🧱️ The authored problem spec — see `two-room-corridor`'s twin for why this, not the asset, is the
/// authority.
pub fn snapshot() -> Wfc3dSnapshot {
    Wfc3dSnapshot {
        schema: WFC3D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        slots: vec![
            Slot3d { id: "bay-0-ground".into(), x: 0.0, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: None },
            Slot3d { id: "bay-0-top".into(), x: 0.0, y: 1.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: None },
            Slot3d { id: "bay-1-ground".into(), x: 1.0, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: None },
            Slot3d { id: "bay-1-top".into(), x: 1.0, y: 1.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: Some("roof".into()) },
        ],
        edges: vec![
            SlotEdge { id: "edge-bay-0-stack".into(), from_slot_id: "bay-0-ground".into(), to_slot_id: "bay-0-top".into(), relation: "above".into() },
            SlotEdge { id: "edge-bay-1-stack".into(), from_slot_id: "bay-1-ground".into(), to_slot_id: "bay-1-top".into(), relation: "above".into() },
            SlotEdge { id: "edge-ground-row".into(), from_slot_id: "bay-0-ground".into(), to_slot_id: "bay-1-ground".into(), relation: "beside".into() },
            SlotEdge { id: "edge-top-row".into(), from_slot_id: "bay-0-top".into(), to_slot_id: "bay-1-top".into(), relation: "beside".into() },
        ],
        tiles: vec![
            Tile { id: "roof".into(), label: Some("Roof".into()), weight: 1.0, media: crate::unit_wedge_media(Some(Color { r: 120, g: 60, b: 50, a: 255 })) },
            Tile { id: "wall".into(), label: Some("Wall".into()), weight: 3.0, media: crate::unit_box_media(Some(Color { r: 210, g: 205, b: 195, a: 255 })) },
        ],
        rules: vec![
            GraphRule { id: "rule-roof-roof".into(), tile_a_id: "roof".into(), tile_b_id: "roof".into(), relation: None, allowed: true },
            GraphRule { id: "rule-roof-roof-beside".into(), tile_a_id: "roof".into(), tile_b_id: "roof".into(), relation: Some("beside".into()), allowed: false },
            GraphRule { id: "rule-wall-roof".into(), tile_a_id: "roof".into(), tile_b_id: "wall".into(), relation: None, allowed: true },
            GraphRule { id: "rule-wall-wall".into(), tile_a_id: "wall".into(), tile_b_id: "wall".into(), relation: None, allowed: true },
        ],
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
