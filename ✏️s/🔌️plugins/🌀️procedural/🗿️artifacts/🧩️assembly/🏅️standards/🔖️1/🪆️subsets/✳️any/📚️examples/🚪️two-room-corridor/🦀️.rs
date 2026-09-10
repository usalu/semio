//! 📚️ Example `two-room-corridor` — the smallest WFC problem that still has a real constraint.
//!
//! Three slots on a line, two adjacency edges, two placeable modules (`room`, `corridor`). The rule
//! set forbids both same-module adjacencies, so the ONLY consistent assignment over this topology is
//! `room · corridor · room` — the example exists to make that forcing visible: the solver has no
//! freedom left, which is what makes its outcome a committable fixture rather than a seeded sample.

use crate::schema::snapshot::{AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge, AssemblySnapshot, ASSEMBLY_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "two-room-corridor";
pub const ICON: &str = "route";
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
pub fn snapshot() -> AssemblySnapshot {
    AssemblySnapshot {
        schema: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        slots: vec![
            AssemblySlot { id: "room-a".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: None },
            AssemblySlot { id: "corridor".into(), x: 1.0, y: 0.0, z: 0.0, pinned_module_id: None },
            AssemblySlot { id: "room-b".into(), x: 2.0, y: 0.0, z: 0.0, pinned_module_id: None },
        ],
        edges: vec![
            AssemblySlotEdge { id: "edge-a-corridor".into(), from_slot_id: "room-a".into(), to_slot_id: "corridor".into() },
            AssemblySlotEdge { id: "edge-corridor-b".into(), from_slot_id: "corridor".into(), to_slot_id: "room-b".into() },
        ],
        modules: vec![crate::module_child_handle("room"), crate::module_child_handle("corridor")],
        weights: vec![AssemblyModuleWeight { module_id: "room".into(), weight: 2.0 }, AssemblyModuleWeight { module_id: "corridor".into(), weight: 1.0 }],
        rules: vec![
            AssemblyRule { id: "rule-room-corridor".into(), module_a_id: "room".into(), module_b_id: "corridor".into(), allowed: true, params: Default::default() },
            AssemblyRule { id: "rule-room-room".into(), module_a_id: "room".into(), module_b_id: "room".into(), allowed: false, params: Default::default() },
            AssemblyRule { id: "rule-corridor-corridor".into(), module_a_id: "corridor".into(), module_b_id: "corridor".into(), allowed: false, params: Default::default() },
        ],
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
