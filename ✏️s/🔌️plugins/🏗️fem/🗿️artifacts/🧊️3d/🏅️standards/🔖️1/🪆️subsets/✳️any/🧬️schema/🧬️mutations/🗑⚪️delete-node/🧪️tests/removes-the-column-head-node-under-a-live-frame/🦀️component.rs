//! 🧪️ `delete-node` fixture — `removes-the-column-head-node-under-a-live-frame`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! `delete-node` is cascade-free: frame `f1` keeps naming `n3` as its end node after the node is gone.

use crate::artifacts::fem3d::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Fem3dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `delete-node` drops `n3` from the node table and carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("delete-node applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-node/removes-the-column-head-node-under-a-live-frame: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.nodes.len(), 2, "delete-node/removes-the-column-head-node-under-a-live-frame: n3 must be gone from the node table");
    assert_eq!(snapshot.elements, before().elements, "delete-node/removes-the-column-head-node-under-a-live-frame: delete-node has no cascade — f1 must survive still naming n3");
    assert_eq!(snapshot.sections, before().sections, "delete-node/removes-the-column-head-node-under-a-live-frame: the section table is untouched by a node deletion");
}

/// ↩️ The inverse is a `create-node` that re-appends `n3` at the tail, restoring the original order.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-node/removes-the-column-head-node-under-a-live-frame: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-node/removes-the-column-head-node-under-a-live-frame: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-node/removes-the-column-head-node-under-a-live-frame: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "delete-node/removes-the-column-head-node-under-a-live-frame: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "delete-node/removes-the-column-head-node-under-a-live-frame: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "delete-node/removes-the-column-head-node-under-a-live-frame: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("delete-node/removes-the-column-head-node-under-a-live-frame: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `nodes.removed` id — no element delta, because this mutation does not cascade.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().nodes.as_ref().expect("nodes delta").removed, vec!["n3".to_string()], "delete-node/removes-the-column-head-node-under-a-live-frame: exactly n3 may be removed");
    assert!(outcome.diff().elements.is_none(), "delete-node/removes-the-column-head-node-under-a-live-frame: no element delta may be opened by a node deletion");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-node/removes-the-column-head-node-under-a-live-frame: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-node/removes-the-column-head-node-under-a-live-frame: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `nodes.removed` id on `before` must leave `f1` dangling but intact.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-node/removes-the-column-head-node-under-a-live-frame: committed diff did not carry before to after");
}
