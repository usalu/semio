//! 💡️ Assembly inferences — THE SOLVE ITSELF IS AN INFERENCE, exactly as the ticket's design
//! ruling states: `AssemblySnapshot` only ever persists the PROBLEM (slots/edges/modules/weights/
//! rules/seed); the SOLUTION, the contradiction/unsat verdict, and the pre-propagation entropy map
//! are all derived here via `store::InferredField`, never mutation-authored state. The 10,930 LOC
//! WFC implementation in the sibling `../🧩️wfc-engine/` compute tree becomes the internals of
//! these `compute()` bodies. Determinism: `solve_with_job` reads only `snapshot` fields (`seed`
//! included) and drives the same resumable `WfcJob` used by interactive callers; every step is
//! watchdog-wrapped and explicitly bounded. No ambient randomness enters the inference, so
//! `DepHash` caching over `AssemblySolve`/`AssemblyContradiction`/`AssemblyEntropy` is sound.

use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;
use std::collections::BTreeMap;

//#region 🔖️Compile
/// 🏗️ Compiles the snapshot problem into the same persistent `WfcJob` used by interactive
/// hosts; the synchronous inference adapter below only drives this job through bounded steps.
fn compile_job(snapshot: &AssemblySnapshot, seed: u64) -> Result<crate::wfc_engine::job::WfcJob<crate::wfc_engine::topology::GraphTopology>, String> {
    let mut builder = crate::wfc_engine::model::ModelBuilder::new();
    let mut pattern_of: BTreeMap<String, crate::wfc_engine::ids::PatternId> = BTreeMap::new();
    for module in &snapshot.modules {
        let weight = snapshot.weights.iter().find(|w| w.module_id == module.child_id).map(|w| w.weight).unwrap_or(1.0);
        pattern_of.insert(module.child_id.clone(), builder.add_pattern(weight));
    }
    let relation = builder.add_relation("adjacent");
    for rule in &snapshot.rules {
        if !rule.allowed {
            continue;
        }
        if let (Some(&a), Some(&b)) = (pattern_of.get(&rule.module_a_id), pattern_of.get(&rule.module_b_id)) {
            builder.allow_mirrored(relation, a, b);
        }
    }
    let model = builder.compile().map_err(|error| format!("{error:?}"))?;

    let node_of: BTreeMap<String, crate::wfc_engine::ids::NodeId> = snapshot.slots.iter().enumerate().map(|(index, slot)| (slot.id.clone(), crate::wfc_engine::ids::NodeId::from_index(index))).collect();
    let mut topology_builder = crate::wfc_engine::topology::GraphTopologyBuilder::new(snapshot.slots.len());
    for edge in &snapshot.edges {
        if let (Some(&from), Some(&to)) = (node_of.get(&edge.from_slot_id), node_of.get(&edge.to_slot_id)) {
            topology_builder.arc(from, to, relation);
            topology_builder.arc(to, from, relation);
        }
    }
    let topology = topology_builder.build().map_err(|error| format!("{error:?}"))?;

    let mut fixed = Vec::new();
    for slot in &snapshot.slots {
        if let Some(pinned) = &slot.pinned_module_id {
            if let (Some(&node), Some(&pattern)) = (node_of.get(&slot.id), pattern_of.get(pinned)) {
                fixed.push((node, pattern));
            }
        }
    }
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), seed);
    Ok(crate::wfc_engine::job::WfcJob::new(operation, model, topology, crate::wfc_engine::job::WfcJobConfig::default(), None, fixed))
}

/// 🏁 Headless inference boundary: repeatedly invokes the shared job driver, whose every
/// `step()` is fuel/deadline bounded and checked by the global 8 ms watchdog.
fn solve_with_job(snapshot: &AssemblySnapshot, seed: u64) -> Result<crate::wfc_engine::job::WfcCommit, String> {
    let mut job = compile_job(snapshot, seed)?;
    let operation = job.operation();
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig { site: "assembly.wfc.inference", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_ms: 2 },
        now_ms: semio_framework_job::default_now_ms,
    };
    match semio_framework_job::run_to_completion(&mut job, &params) {
        semio_framework_job::StepOutcome::Complete(_) => job.commit().ok_or_else(|| "wfc-completed-without-commit".to_string()),
        semio_framework_job::StepOutcome::Cancelled => Err("wfc-cancelled".to_string()),
        semio_framework_job::StepOutcome::Fault(fault) => Err(String::from_utf8_lossy(&fault.detail).into_owned()),
        outcome => Err(format!("wfc-batch-driver-returned-nonterminal:{outcome:?}")),
    }
}

fn module_id_at_pattern_index(snapshot: &AssemblySnapshot, pattern: u32) -> Option<String> {
    snapshot.modules.get(pattern as usize).map(|module| module.child_id.clone())
}
//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁 The solved assignment (slot id → module id), or `Unsolved` for every non-`Solved` outcome
/// (`Unsatisfiable`/`Contradiction`/budget/cancellation) — see `AssemblyContradiction` for the
/// dedicated satisfiability verdict.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssemblySolveResult {
    #[default]
    Unsolved,
    Solved {
        assignments: BTreeMap<String, String>,
    },
}

pub struct AssemblySolve;

impl store::InferredField<AssemblySnapshot> for AssemblySolve {
    type Key = String;
    type Value = AssemblySolveResult;

    const FIELD_ID: &'static str = "s.assembly.inference.solve";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "modules", "weights", "rules"]
    }
    fn plan(_snapshot: &AssemblySnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "assembly".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &AssemblySnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        serde_json::to_vec(snapshot).expect("AssemblySnapshot serialization never fails")
    }
    fn compute(snapshot: &AssemblySnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot, snapshot.seed) {
            Ok(solution) => {
                let assignments = snapshot.slots.iter().enumerate().filter_map(|(index, slot)| solution.assignment.get(index).and_then(|&pattern| module_id_at_pattern_index(snapshot, pattern)).map(|module_id| (slot.id.clone(), module_id))).collect();
                AssemblySolveResult::Solved { assignments }
            }
            _ => AssemblySolveResult::Unsolved,
        }
    }
}
//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺 The satisfiability verdict on its own — the natural sibling to `AssemblySolve` the design
/// calls out explicitly, so a caller who only needs "is this spec even solvable" never has to
/// decode a full assignment map to find out.
pub struct AssemblyContradiction;

impl store::InferredField<AssemblySnapshot> for AssemblyContradiction {
    type Key = String;
    type Value = bool;

    const FIELD_ID: &'static str = "s.assembly.inference.contradiction";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "slots", "edges", "modules", "weights", "rules"]
    }
    fn plan(_snapshot: &AssemblySnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "assembly".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &AssemblySnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        serde_json::to_vec(snapshot).expect("AssemblySnapshot serialization never fails")
    }
    fn compute(snapshot: &AssemblySnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        solve_with_job(snapshot, snapshot.seed).is_ok()
    }
}
//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 Per-slot Shannon entropy of the module WEIGHT distribution — `0.0` for a `pinned_module_id`
/// slot (fully determined), else the prior entropy over every module's `AssemblyModuleWeight`
/// (neutral `1.0` when a module has no explicit weight entry). SCOPE, honestly stated: this is the
/// PRIOR entropy before arc-consistency propagation narrows any slot's domain — a real, useful WFC
/// heuristic (the same weighted-distribution math `wfc_engine::weights::WeightTable` encodes), but
/// not the POST-propagation entropy a live "which cell should I collapse next" UI would want; wiring
/// this field through `wfc_engine::propagate`/`prop_ac3` for a truly narrowed per-slot domain is a
/// real remaining increment, not done here.
pub struct AssemblyEntropy;

impl store::InferredField<AssemblySnapshot> for AssemblyEntropy {
    type Key = String;
    type Value = f64;

    const FIELD_ID: &'static str = "s.assembly.inference.entropy";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["slots", "modules", "weights"]
    }
    fn plan(snapshot: &AssemblySnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot.slots.iter().map(|slot| store::InferenceStep { key: slot.id.clone(), parents: Vec::new() }).collect()
    }
    fn dep_input(snapshot: &AssemblySnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let pinned = snapshot.slots.iter().find(|slot| &slot.id == key).and_then(|slot| slot.pinned_module_id.clone());
        bytes.extend_from_slice(pinned.unwrap_or_default().as_bytes());
        bytes.push(0);
        for module in &snapshot.modules {
            bytes.extend_from_slice(module.child_id.as_bytes());
            bytes.push(0);
        }
        for weight in &snapshot.weights {
            bytes.extend_from_slice(weight.module_id.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&weight.weight.to_le_bytes());
        }
        bytes
    }
    fn compute(snapshot: &AssemblySnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let pinned = snapshot.slots.iter().find(|slot| &slot.id == key).and_then(|slot| slot.pinned_module_id.as_ref());
        if pinned.is_some() {
            return 0.0;
        }
        shannon_entropy_over_modules(snapshot)
    }
}

fn shannon_entropy_over_modules(snapshot: &AssemblySnapshot) -> f64 {
    let weights: Vec<f64> = snapshot.modules.iter().map(|module| snapshot.weights.iter().find(|w| w.module_id == module.child_id).map(|w| w.weight).unwrap_or(1.0)).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|w| w / total).filter(|p| *p > 0.0).map(|p| p * p.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::assembly::schema::snapshot::{AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    use store::InferredField;

    fn kit_child(id: &str) -> store::ArtifactChild<SemioKitSnapshot> {
        store::ArtifactChild::new(id.to_string(), store::os_io::ArtifactRef { artifact_id: id.to_string(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() } })
    }

    /// 🧸 Two slots, one edge, two modules ("a","b") mutually allowed to be adjacent — a WFC
    /// instance small enough to solve deterministically by hand: any seed must find SOME solution.
    fn two_slot_two_module_snapshot() -> AssemblySnapshot {
        let mut snapshot = AssemblySnapshot::default();
        snapshot.seed = 7;
        snapshot.slots = vec![AssemblySlot { id: "s1".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: None }, AssemblySlot { id: "s2".into(), x: 1.0, y: 0.0, z: 0.0, pinned_module_id: None }];
        snapshot.edges = vec![AssemblySlotEdge { id: "e1".into(), from_slot_id: "s1".into(), to_slot_id: "s2".into() }];
        snapshot.modules = vec![kit_child("a"), kit_child("b")];
        snapshot.rules = vec![AssemblyRule { id: "r1".into(), module_a_id: "a".into(), module_b_id: "b".into(), allowed: true, params: SemioValue::default() }];
        snapshot
    }

    #[test]
    fn solve_over_an_always_allowed_pair_finds_an_assignment_for_every_slot() {
        let snapshot = two_slot_two_module_snapshot();
        let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        match &values["assembly"] {
            AssemblySolveResult::Solved { assignments } => assert_eq!(assignments.len(), 2, "both slots must be assigned"),
            AssemblySolveResult::Unsolved => panic!("a trivially satisfiable spec must solve"),
        }
    }

    #[test]
    fn contradiction_field_agrees_with_solve_field_on_a_satisfiable_spec() {
        let snapshot = two_slot_two_module_snapshot();
        let satisfiable = store::infer_field::<AssemblySnapshot, AssemblyContradiction>(&snapshot, None);
        assert_eq!(satisfiable["assembly"], true);
    }

    #[test]
    fn an_unsatisfiable_spec_is_reported_as_a_contradiction_not_a_panic() {
        let mut snapshot = two_slot_two_module_snapshot();
        // 🚫 No rule allows "a" next to "a" or "b" next to "b" AND no rule allows "a"-"b" either
        // once we remove it — an edge with a fully closed-world empty allow-set is unsatisfiable.
        snapshot.rules.clear();
        let satisfiable = store::infer_field::<AssemblySnapshot, AssemblyContradiction>(&snapshot, None);
        assert_eq!(satisfiable["assembly"], false);
        let solved = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        assert_eq!(solved["assembly"], AssemblySolveResult::Unsolved);
    }

    #[test]
    fn pinned_slot_always_resolves_to_its_pinned_module() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.slots[0].pinned_module_id = Some("a".into());
        snapshot.slots[1].pinned_module_id = Some("b".into());
        let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        match &values["assembly"] {
            AssemblySolveResult::Solved { assignments } => {
                assert_eq!(assignments["s1"], "a");
                assert_eq!(assignments["s2"], "b");
            }
            AssemblySolveResult::Unsolved => panic!("a pinned-and-allowed pair must solve"),
        }
    }

    #[test]
    fn empty_assembly_solves_trivially_with_no_assignments() {
        let snapshot = AssemblySnapshot::default();
        let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        assert_eq!(values["assembly"], AssemblySolveResult::Solved { assignments: BTreeMap::new() });
    }

    #[test]
    fn pinned_slot_has_zero_entropy_unpinned_slot_does_not() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.slots[0].pinned_module_id = Some("a".into());
        let entropy = store::infer_field::<AssemblySnapshot, AssemblyEntropy>(&snapshot, None);
        assert_eq!(entropy["s1"], 0.0);
        assert!(entropy["s2"] > 0.0, "an unpinned slot over two equally-weighted modules must have positive entropy");
    }

    #[test]
    fn uniform_weights_over_two_modules_yield_ln2_entropy() {
        let snapshot = two_slot_two_module_snapshot();
        let entropy = store::infer_field::<AssemblySnapshot, AssemblyEntropy>(&snapshot, None);
        assert!((entropy["s1"] - std::f64::consts::LN_2).abs() < 1e-9);
    }

    #[test]
    fn skewed_weights_lower_entropy_than_uniform() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.weights = vec![AssemblyModuleWeight { module_id: "a".into(), weight: 100.0 }, AssemblyModuleWeight { module_id: "b".into(), weight: 0.01 }];
        let entropy = store::infer_field::<AssemblySnapshot, AssemblyEntropy>(&snapshot, None);
        assert!(entropy["s1"] < std::f64::consts::LN_2, "a skewed distribution must have lower entropy than the uniform case");
    }

    /// 🔁 Determinism law: identical snapshots (same seed) must produce byte-identical solve
    /// results — `InferredField::compute` must be a pure function of `snapshot`, WFC's internal
    /// randomness notwithstanding, since the seed itself lives in the snapshot.
    #[test]
    fn identical_seed_and_spec_always_produce_the_same_solution() {
        let snapshot = two_slot_two_module_snapshot();
        let first = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        let second = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        assert_eq!(first, second);
    }

    #[test]
    fn changing_only_the_seed_still_solves_a_trivially_satisfiable_spec() {
        let mut snapshot = two_slot_two_module_snapshot();
        snapshot.seed = 999;
        let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
        assert!(matches!(values["assembly"], AssemblySolveResult::Solved { .. }));
    }
}
//#endregion 🧪️Tests
