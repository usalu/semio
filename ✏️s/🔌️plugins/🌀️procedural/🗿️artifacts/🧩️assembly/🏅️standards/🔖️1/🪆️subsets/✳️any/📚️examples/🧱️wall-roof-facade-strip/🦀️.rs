//! 📚️ Example `wall-roof-facade-strip` — a two-storey facade strip, the smallest problem in which
//! the topology is a CYCLE rather than a path.
//!
//! Four slots in a 2×2 lattice (two ground bays, two roof bays above them), four adjacency edges.
//! `roof` may not sit beside `roof`, so the two roof bays cannot both be roof — and since the ground
//! row is unconstrained against itself, the spec stays satisfiable while still forcing a choice.
//! Together with `two-room-corridor` (a forced path) this covers the two topologies the WFC engine's
//! propagation treats differently: acyclic and cyclic.

use crate::schema::snapshot::{AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge, AssemblySnapshot, ASSEMBLY_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "wall-roof-facade-strip";
pub const ICON: &str = "brick-wall";
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
pub fn snapshot() -> AssemblySnapshot {
    AssemblySnapshot {
        schema: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        seed: SEED,
        slots: vec![
            AssemblySlot { id: "bay-0-ground".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: None },
            AssemblySlot { id: "bay-1-ground".into(), x: 1.0, y: 0.0, z: 0.0, pinned_module_id: None },
            AssemblySlot { id: "bay-0-top".into(), x: 0.0, y: 0.0, z: 1.0, pinned_module_id: None },
            AssemblySlot { id: "bay-1-top".into(), x: 1.0, y: 0.0, z: 1.0, pinned_module_id: Some("roof".into()) },
        ],
        edges: vec![
            AssemblySlotEdge { id: "edge-ground-row".into(), from_slot_id: "bay-0-ground".into(), to_slot_id: "bay-1-ground".into() },
            AssemblySlotEdge { id: "edge-top-row".into(), from_slot_id: "bay-0-top".into(), to_slot_id: "bay-1-top".into() },
            AssemblySlotEdge { id: "edge-bay-0-stack".into(), from_slot_id: "bay-0-ground".into(), to_slot_id: "bay-0-top".into() },
            AssemblySlotEdge { id: "edge-bay-1-stack".into(), from_slot_id: "bay-1-ground".into(), to_slot_id: "bay-1-top".into() },
        ],
        modules: vec![crate::module_child_handle("wall"), crate::module_child_handle("roof")],
        weights: vec![AssemblyModuleWeight { module_id: "wall".into(), weight: 3.0 }, AssemblyModuleWeight { module_id: "roof".into(), weight: 1.0 }],
        rules: vec![
            AssemblyRule { id: "rule-wall-wall".into(), module_a_id: "wall".into(), module_b_id: "wall".into(), allowed: true, params: Default::default() },
            AssemblyRule { id: "rule-wall-roof".into(), module_a_id: "wall".into(), module_b_id: "roof".into(), allowed: true, params: Default::default() },
            AssemblyRule { id: "rule-roof-roof".into(), module_a_id: "roof".into(), module_b_id: "roof".into(), allowed: false, params: Default::default() },
        ],
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
