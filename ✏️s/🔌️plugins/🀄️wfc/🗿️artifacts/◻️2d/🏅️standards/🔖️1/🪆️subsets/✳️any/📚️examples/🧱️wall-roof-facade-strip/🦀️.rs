//! 🧱️ Example `wall-roof-facade-strip` — a two-storey facade strip, the smallest problem whose
//! topology is a CYCLE rather than a path.
//!
//! Four slots in a 2×2 lattice (two ground bays, two roof bays above them) and four adjacency edges,
//! authored across TWO relation classes: `beside` along a row and `above` up a stack. `roof` may not
//! sit beside `roof`, and `roof` may only sit ABOVE a wall — a rule scoped to one relation, which is
//! exactly what this artifact adds over assembly's single-relation ancestor.

use crate::schema::snapshot::{Wfc2dColor, Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile, WFC_2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "wall-roof-facade-strip";
pub const ICON: &str = "building";
pub const SEED: u64 = 42;
/// 🔗 The two adjacency classes this example authors.
pub const RELATION_BESIDE: &str = "beside";
pub const RELATION_ABOVE: &str = "above";

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Wall And Roof Facade Strip", "Wand-Dach-Fassadenstreifen")
}

fn slot(id: &str, x: f64, y: f64, pinned: Option<&str>) -> Wfc2dSlot {
    Wfc2dSlot { id: id.into(), x, y, width: 2.0, height: 2.0, pinned_tile_id: pinned.map(str::to_string) }
}

fn edge(id: &str, from: &str, to: &str, relation: &str) -> Wfc2dSlotEdge {
    Wfc2dSlotEdge { id: id.into(), from_slot_id: from.into(), to_slot_id: to.into(), relation: relation.into() }
}

/// 🧱️ The authored problem spec — see `two-room-corridor`'s twin for why this, not a text asset, is
/// the authority.
pub fn document() -> Wfc2dSnapshot {
    Wfc2dSnapshot {
        schema: WFC_2D_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        slots: vec![slot("bay-0-ground", 0.0, 2.0, None), slot("bay-0-top", 0.0, 0.0, None), slot("bay-1-ground", 2.0, 2.0, None), slot("bay-1-top", 2.0, 0.0, Some("roof"))],
        edges: vec![
            edge("edge-bay-0-stack", "bay-0-ground", "bay-0-top", RELATION_ABOVE),
            edge("edge-bay-1-stack", "bay-1-ground", "bay-1-top", RELATION_ABOVE),
            edge("edge-ground-row", "bay-0-ground", "bay-1-ground", RELATION_BESIDE),
            edge("edge-top-row", "bay-0-top", "bay-1-top", RELATION_BESIDE),
        ],
        tiles: vec![
            Wfc2dTile { id: "roof".into(), label: Some("Roof".into()), weight: 1.0, media: super::two_room_corridor::filled_square(Wfc2dColor { r: 206, g: 84, b: 62, a: 255 }) },
            Wfc2dTile { id: "wall".into(), label: Some("Wall".into()), weight: 3.0, media: super::two_room_corridor::filled_square(Wfc2dColor { r: 122, g: 126, b: 134, a: 255 }) },
        ],
        rules: vec![
            Wfc2dRule { id: "rule-roof-beside-roof".into(), tile_a_id: "roof".into(), tile_b_id: "roof".into(), relation: Some(RELATION_BESIDE.into()), allowed: false },
            Wfc2dRule { id: "rule-wall-roof".into(), tile_a_id: "wall".into(), tile_b_id: "roof".into(), relation: None, allowed: true },
            Wfc2dRule { id: "rule-wall-wall".into(), tile_a_id: "wall".into(), tile_b_id: "wall".into(), relation: None, allowed: true },
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
