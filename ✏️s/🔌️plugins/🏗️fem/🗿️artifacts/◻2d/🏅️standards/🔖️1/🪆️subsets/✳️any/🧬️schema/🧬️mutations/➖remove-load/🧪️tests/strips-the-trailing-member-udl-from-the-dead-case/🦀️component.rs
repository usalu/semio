//! 🧪️ `remove-load` fixture — `strips-the-trailing-member-udl-from-the-dead-case`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! The payload names only ids; the removed UDL itself is recovered from `base` by the inverse, never carried in the mutation.

use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Fem2dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `remove-load` detaches `l2` and carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("remove-load applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "remove-load/strips-the-trailing-member-udl-from-the-dead-case: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.load_cases[0].loads.len(), 1, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: only the nodal load may remain");
    assert_eq!(crate::artifacts::fem2d::load_id(&snapshot.load_cases[0].loads[0]), "l1", "remove-load/strips-the-trailing-member-udl-from-the-dead-case: the surviving load is the nodal one");
    assert_eq!(snapshot.elements, before().elements, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: the element the UDL sat on must stay");
}

/// ↩️ The inverse is an `add-load` rebuilt from `base`, re-appending the UDL it never stored itself.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "remove-load/strips-the-trailing-member-udl-from-the-dead-case: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("remove-load/strips-the-trailing-member-udl-from-the-dead-case: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be one `loadCases.patched` entry whose item has lost exactly the named load.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().load_cases.as_ref().expect("loadCases delta").patched.len(), 1, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: the owning case must be patched exactly once");
    assert!(outcome.diff().load_cases.as_ref().expect("loadCases delta").removed.is_empty(), "remove-load/strips-the-trailing-member-udl-from-the-dead-case: detaching a load must never remove the case");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-load/strips-the-trailing-member-udl-from-the-dead-case: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `loadCases.patched` entry on `before` must yield the single-load case.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-load/strips-the-trailing-member-udl-from-the-dead-case: committed diff did not carry before to after");
}
