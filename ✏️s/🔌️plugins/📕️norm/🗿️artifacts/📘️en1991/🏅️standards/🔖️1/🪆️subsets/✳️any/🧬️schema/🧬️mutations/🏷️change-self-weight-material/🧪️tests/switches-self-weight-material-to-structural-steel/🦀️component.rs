//! 🧪️ `change-self-weight-material` fixture — `switches-self-weight-material-to-structural-steel` (EN 1991 actions).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1991Snapshot {
    serde_json::from_str(BEFORE).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: mutation decodes")
}

/// ▶️ `change-self-weight-material` carries `self_weight_material` from reinforced_concrete to structural_steel and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: mutation applies to its committed before-snapshot");
    assert_eq!(produced.self_weight_material, "structural_steel", "change-self-weight-material/switches-self-weight-material-to-structural-steel: `self_weight_material` must read structural_steel after the mutation");
    assert_eq!(produced.self_weight_thickness_m, base.self_weight_thickness_m, "change-self-weight-material/switches-self-weight-material-to-structural-steel: `self_weight_thickness_m` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-self-weight-material/switches-self-weight-material-to-structural-steel: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `self_weight_material` (reinforced_concrete) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: inverse step applies");
    }
    assert_eq!(snapshot.self_weight_material, base.self_weight_material, "change-self-weight-material/switches-self-weight-material-to-structural-steel: inverse must put `self_weight_material` back to reinforced_concrete");
    assert_eq!(snapshot, base, "change-self-weight-material/switches-self-weight-material-to-structural-steel: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: snapshot reparses");
        assert_eq!(reencoded, original, "change-self-weight-material/switches-self-weight-material-to-structural-steel: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: mutation reparses");
    assert_eq!(reencoded, original, "change-self-weight-material/switches-self-weight-material-to-structural-steel: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean reinforced_concrete→structural_steel edit of `self_weight_material` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-self-weight-material/switches-self-weight-material-to-structural-steel: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-self-weight-material/switches-self-weight-material-to-structural-steel: changing `self_weight_material` away from reinforced_concrete must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-self-weight-material/switches-self-weight-material-to-structural-steel: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `selfWeightMaterial` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().self_weight_material.as_deref(), Some("structural_steel"), "change-self-weight-material/switches-self-weight-material-to-structural-steel: the diff must carry `self_weight_material` = structural_steel");
    assert!(outcome.diff().self_weight_thickness_m.is_none(), "change-self-weight-material/switches-self-weight-material-to-structural-steel: the diff must leave `self_weight_thickness_m` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: committed diff decodes");
    assert_eq!(produced, committed, "change-self-weight-material/switches-self-weight-material-to-structural-steel: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: committed diff decodes");
    assert_eq!(decoded.self_weight_material.as_deref(), Some("structural_steel"), "change-self-weight-material/switches-self-weight-material-to-structural-steel: the committed diff must name `self_weight_material` = structural_steel");
    let reencoded = serde_json::to_value(&decoded).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: committed diff reparses");
    assert_eq!(reencoded, original, "change-self-weight-material/switches-self-weight-material-to-structural-steel: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the structural_steel `self_weight_material` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-self-weight-material/switches-self-weight-material-to-structural-steel: committed diff applies to the before-snapshot");
    assert_eq!(produced.self_weight_material, "structural_steel", "change-self-weight-material/switches-self-weight-material-to-structural-steel: the committed diff must set `self_weight_material` to structural_steel");
    assert_eq!(produced, expected_after(), "change-self-weight-material/switches-self-weight-material-to-structural-steel: committed diff did not carry before to after");
}
