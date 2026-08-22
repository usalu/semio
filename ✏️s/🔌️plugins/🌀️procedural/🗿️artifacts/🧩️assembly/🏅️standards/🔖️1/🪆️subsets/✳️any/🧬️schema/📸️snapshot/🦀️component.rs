//! 🧩️ AssemblySnapshot — semio's WFC-solved Assembly: SLOTS (topology nodes to fill), EDGES (the
//! generic adjacency graph WFC propagates over — no 2D/3D grid assumption baked in), MODULES (the
//! catalog of placeable content each slot may resolve to, composed via `kit` — owned, per this
//! wave's design ruling: no private closed `Module` type), WEIGHTS (per-module selection bias), and
//! RULES (adjacency constraints between modules, `value`-shaped structured data — never a bespoke
//! struct-per-constraint-kind). The SOLVE itself is never stored here: it is an INFERENCE
//! (`../💡️inferences/🦀️component.rs`) over this spec, never mutation-authored persisted state —
//! only the PROBLEM is authored, the SOLUTION is derived.

use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const ASSEMBLY_DOCUMENT_SCHEMA: &str = "s.assembly";
//#endregion 🔖️Ids

//#region 🔖️Slot
/// 📍 One position in the assembly's topology — a WFC solver variable. `id` is stable/user-authored;
/// the solved module assignment lives in the inference result, never here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssemblySlot {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// 🔒 Optional user-pinned module id — a hard pre-assignment WFC must respect (a domain
    /// restriction feeding the solver, never overwritten by it).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_module_id: Option<String>,
}

/// 🔗 One adjacency EDGE between two slots — the generic graph topology WFC propagates constraints
/// over (`graph_core::GraphView`), independent of any regular-grid assumption.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssemblySlotEdge {
    pub id: String,
    pub from_slot_id: String,
    pub to_slot_id: String,
}
//#endregion 🔖️Slot

//#region 🔖️Weight
/// ⚖️ Per-module selection bias (`wfc_engine::weights::WeightTable` input) — id-keyed, upserted by
/// `change-weight`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssemblyModuleWeight {
    pub module_id: String,
    pub weight: f64,
}
//#endregion 🔖️Weight

//#region 🔖️Rule
/// ⛓️ One ADJACENCY RULE between two module ids across an edge — `params` is `value`-shaped
/// structured data (a `SemioValue`), reusing the same generic vocabulary `kit`'s own `properties`
/// slot composes rather than minting a private closed type per constraint kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssemblyRule {
    pub id: String,
    pub module_a_id: String,
    pub module_b_id: String,
    pub allowed: bool,
    #[serde(default)]
    pub params: SemioValue,
}
//#endregion 🔖️Rule

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.assembly")]
pub struct AssemblySnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🎲 Deterministic solve seed — PERSISTED, authored only via `change-seed`, never ambient: the
    /// solve inference's `DepHash` caching is sound only if `compute` is a pure function of
    /// snapshot content, seed included (WFC is seeded-random internally).
    #[state(artifact)]
    pub seed: u64,
    #[state(artifact)]
    #[serde(default)]
    pub slots: Vec<AssemblySlot>,
    #[state(artifact)]
    #[serde(default)]
    pub edges: Vec<AssemblySlotEdge>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.kit")]
    #[serde(default)]
    pub modules: Vec<store::ArtifactChild<SemioKitSnapshot>>,
    #[state(artifact)]
    #[serde(default)]
    pub weights: Vec<AssemblyModuleWeight>,
    #[state(artifact)]
    #[serde(default)]
    pub rules: Vec<AssemblyRule>,
}

impl Default for AssemblySnapshot {
    fn default() -> Self {
        Self { schema: ASSEMBLY_DOCUMENT_SCHEMA.into(), seed: 0, slots: Vec::new(), edges: Vec::new(), modules: Vec::new(), weights: Vec::new(), rules: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Addressing
pub fn slot_index(snapshot: &AssemblySnapshot, id: &str) -> Option<usize> {
    snapshot.slots.iter().position(|slot| slot.id == id)
}
pub fn edge_index(snapshot: &AssemblySnapshot, id: &str) -> Option<usize> {
    snapshot.edges.iter().position(|edge| edge.id == id)
}
pub fn rule_index(snapshot: &AssemblySnapshot, id: &str) -> Option<usize> {
    snapshot.rules.iter().position(|rule| rule.id == id)
}
pub fn weight_index(snapshot: &AssemblySnapshot, module_id: &str) -> Option<usize> {
    snapshot.weights.iter().position(|weight| weight.module_id == module_id)
}
//#endregion 🔖️Addressing

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_snapshot_is_empty_and_zero_seeded() {
        let snapshot = AssemblySnapshot::default();
        assert_eq!(snapshot.schema, ASSEMBLY_DOCUMENT_SCHEMA);
        assert_eq!(snapshot.seed, 0);
        assert!(snapshot.slots.is_empty() && snapshot.edges.is_empty() && snapshot.modules.is_empty() && snapshot.weights.is_empty() && snapshot.rules.is_empty());
    }

    #[test]
    fn json_round_trips() {
        let mut snapshot = AssemblySnapshot::default();
        snapshot.slots.push(AssemblySlot { id: "s1".into(), x: 1.0, y: 2.0, z: 0.0, pinned_module_id: None });
        snapshot.edges.push(AssemblySlotEdge { id: "e1".into(), from_slot_id: "s1".into(), to_slot_id: "s1".into() });
        snapshot.weights.push(AssemblyModuleWeight { module_id: "m1".into(), weight: 2.5 });
        snapshot.rules.push(AssemblyRule { id: "r1".into(), module_a_id: "m1".into(), module_b_id: "m2".into(), allowed: true, params: SemioValue::default() });
        let bytes = serde_json::to_vec(&snapshot).expect("encode");
        let back: AssemblySnapshot = serde_json::from_slice(&bytes).expect("decode");
        assert_eq!(snapshot, back);
    }

    #[test]
    fn addressing_finds_existing_and_misses_unknown() {
        let mut snapshot = AssemblySnapshot::default();
        snapshot.slots.push(AssemblySlot { id: "s1".into(), ..Default::default() });
        assert_eq!(slot_index(&snapshot, "s1"), Some(0));
        assert_eq!(slot_index(&snapshot, "missing"), None);
    }
}
//#endregion 🧪️Tests
