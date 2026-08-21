//! 🧪️ `delete-support` fixture — `releases-the-roller-at-node-n2`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Releasing the roller leaves the model under-restrained on purpose — the mutation layer never second-guesses stability.

use crate::artifacts::fem2d::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::artifacts::fem2d::mutations::Fem2dMutation;
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

/// ▶️ `delete-support` drops the roller `s2` and carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-support applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-support/releases-the-roller-at-node-n2: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.supports.len(), 1, "delete-support/releases-the-roller-at-node-n2: only the pin may remain");
    assert_eq!(snapshot.supports[0].id, "s1", "delete-support/releases-the-roller-at-node-n2: the pin is the survivor");
    assert_eq!(snapshot.nodes, before().nodes, "delete-support/releases-the-roller-at-node-n2: releasing a support never deletes the node it sat on");
}

/// ↩️ The inverse is a `create-support` rebuilt from `base`, re-appending the roller with its DOF list.
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
    assert_eq!(snapshot, base, "delete-support/releases-the-roller-at-node-n2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-support/releases-the-roller-at-node-n2: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-support/releases-the-roller-at-node-n2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "delete-support/releases-the-roller-at-node-n2: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "delete-support/releases-the-roller-at-node-n2: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "delete-support/releases-the-roller-at-node-n2: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("delete-support/releases-the-roller-at-node-n2: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `supports.removed` id — node `n2` itself stays put.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().supports.as_ref().expect("supports delta").removed, vec!["s2".to_string()], "delete-support/releases-the-roller-at-node-n2: exactly s2 may be removed");
    assert!(outcome.diff().nodes.is_none(), "delete-support/releases-the-roller-at-node-n2: no node delta may be opened");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-support/releases-the-roller-at-node-n2: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-support/releases-the-roller-at-node-n2: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `supports.removed` id on `before` must leave the pin alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-support/releases-the-roller-at-node-n2: committed diff did not carry before to after");
}
