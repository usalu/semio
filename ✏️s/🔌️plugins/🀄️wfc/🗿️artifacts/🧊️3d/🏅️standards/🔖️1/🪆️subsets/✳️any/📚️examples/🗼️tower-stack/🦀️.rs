//! 📚️ Example `tower-stack` — a deliberately NON-BOXED graph: a three-storey core with one
//! cantilever hanging off the middle storey, which no regular grid can express.
//!
//! Five slots, five edges over two relations (`above` up the core, `beside` out to the cantilever),
//! and differently-sized boxes (the cantilever is a third the height of a core storey). `cap` is
//! admitted ONLY `above` something, so it can never land on the cantilever — the one slot every path
//! reaches through a `beside` edge — and `pier` above `pier` is admitted for every relation and then
//! DENIED for `above`, which forces the storey over the pinned base to be a deck. This is the example
//! the 3d preview exists for: the slot boxes differ, so the same tile mesh is scaled differently per
//! instance.

use crate::schema::snapshot::{Color, GraphRule, Slot3d, SlotEdge, Tile, Wfc3dSnapshot, WFC3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "tower-stack";
pub const ICON: &str = "building";
pub const SEED: u64 = 19;

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Tower With A Cantilever", "Turm mit Auskragung")
}

pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗼️tower-stack/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🗼️ The authored problem spec — the authority its `🗣️.dsl.semio` asset is printed from.
pub fn snapshot() -> Wfc3dSnapshot {
    Wfc3dSnapshot {
        schema: WFC3D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        slots: vec![
            Slot3d { id: "cantilever".into(), x: 1.5, y: 3.0, z: 0.0, width: 2.0, height: 1.0, depth: 1.0, pinned_tile_id: None },
            Slot3d { id: "storey-0".into(), x: 0.0, y: 0.0, z: 0.0, width: 2.0, height: 3.0, depth: 2.0, pinned_tile_id: Some("pier".into()) },
            Slot3d { id: "storey-1".into(), x: 0.0, y: 3.0, z: 0.0, width: 2.0, height: 3.0, depth: 2.0, pinned_tile_id: None },
            Slot3d { id: "storey-2".into(), x: 0.0, y: 6.0, z: 0.0, width: 2.0, height: 3.0, depth: 2.0, pinned_tile_id: None },
            Slot3d { id: "storey-3".into(), x: 0.0, y: 9.0, z: 0.0, width: 2.0, height: 1.0, depth: 2.0, pinned_tile_id: None },
        ],
        edges: vec![
            SlotEdge { id: "edge-0-1".into(), from_slot_id: "storey-0".into(), to_slot_id: "storey-1".into(), relation: "above".into() },
            SlotEdge { id: "edge-1-2".into(), from_slot_id: "storey-1".into(), to_slot_id: "storey-2".into(), relation: "above".into() },
            SlotEdge { id: "edge-2-3".into(), from_slot_id: "storey-2".into(), to_slot_id: "storey-3".into(), relation: "above".into() },
            SlotEdge { id: "edge-cantilever".into(), from_slot_id: "storey-1".into(), to_slot_id: "cantilever".into(), relation: "beside".into() },
            SlotEdge { id: "edge-cantilever-brace".into(), from_slot_id: "storey-2".into(), to_slot_id: "cantilever".into(), relation: "beside".into() },
        ],
        tiles: vec![
            Tile { id: "cap".into(), label: Some("Cap".into()), weight: 1.0, media: crate::unit_wedge_media(Some(Color { r: 90, g: 90, b: 110, a: 255 })) },
            Tile { id: "deck".into(), label: Some("Deck".into()), weight: 3.0, media: crate::unit_box_media(Some(Color { r: 200, g: 190, b: 160, a: 255 })) },
            Tile { id: "pier".into(), label: Some("Pier".into()), weight: 2.0, media: crate::unit_box_media(Some(Color { r: 130, g: 130, b: 135, a: 255 })) },
        ],
        rules: vec![
            GraphRule { id: "rule-cap-deck-above".into(), tile_a_id: "cap".into(), tile_b_id: "deck".into(), relation: Some("above".into()), allowed: true },
            GraphRule { id: "rule-cap-pier-above".into(), tile_a_id: "cap".into(), tile_b_id: "pier".into(), relation: Some("above".into()), allowed: true },
            GraphRule { id: "rule-deck-deck".into(), tile_a_id: "deck".into(), tile_b_id: "deck".into(), relation: None, allowed: true },
            GraphRule { id: "rule-deck-pier".into(), tile_a_id: "deck".into(), tile_b_id: "pier".into(), relation: None, allowed: true },
            GraphRule { id: "rule-pier-pier".into(), tile_a_id: "pier".into(), tile_b_id: "pier".into(), relation: None, allowed: true },
            GraphRule { id: "rule-pier-pier-above".into(), tile_a_id: "pier".into(), tile_b_id: "pier".into(), relation: Some("above".into()), allowed: false },
        ],
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
