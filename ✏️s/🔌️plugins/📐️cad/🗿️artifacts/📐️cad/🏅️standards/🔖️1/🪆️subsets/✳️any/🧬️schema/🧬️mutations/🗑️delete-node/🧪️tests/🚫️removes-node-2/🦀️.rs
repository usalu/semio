//! 🧪️ `delete-node` fixture — `🚫️removes-node-2`.
//!
//! Proves a node id leaves the collection and that undo re-materializes the full record.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> CadSnapshot {
    serde_json::from_str(BEFORE).expect("delete-node/removes-node-2: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("delete-node/removes-node-2: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("delete-node/removes-node-2: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("delete-node applies to its committed before-snapshot")
}

/// ▶️ `delete-node` removes only the addressed node — sibling nodes and every other lane are untouched.
#[semio_framework_async_macros::async_test]
async fn drops_node_2_and_keeps_the_root_node() {
    let after = applied();
    assert_eq!(after.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), vec!["node-1"], "delete-node must remove node-2 and only node-2");
    assert_eq!(after.nodes[0].label, "Root", "delete-node must not relabel the surviving node");
    assert_eq!(after.drawings.len(), 1, "delete-node must not cascade into the drawings collection");
    assert_eq!(after, expected_after(), "delete-node/removes-node-2: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `create-node` carrying the ENTIRE removed record, not just its id.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_node_2_with_its_label_and_kind() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "delete-node inverts to exactly one step");
    match &inverse[0] {
        CadMutation::CreateNode(step) => {
            assert_eq!(step.node.id, "node-2", "the inverse must recreate the removed node");
            assert_eq!(step.node.label, "Base Plate", "the inverse must carry the removed node's label, not a stub");
            assert_eq!(step.node.kind, "solid", "the inverse must carry the removed node's kind");
        }
        other => panic!("delete-node must invert to create-node, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("delete-node/removes-node-2: inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-node/removes-node-2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-node/removes-node-2: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-node/removes-node-2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-node`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-node/removes-node-2: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "delete-node/removes-node-2: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().nodes.as_ref().expect("delete-node fills the nodes delta");
    assert_eq!(delta.removed, vec!["node-2".to_string()], "delete-node's diff carries the id in `removed`");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-node touches only the `removed` arm of the nodes delta");
}

/// 🔺️ The sparse delta `delete-node` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `nodes.removed` is populated, carrying the bare id — the removed record lives in the INVERSE.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-node/removes-node-2: delete-node must emit a nodes delta whose only populated arm is `removed`");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-node/removes-node-2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `delete-node` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-node/removes-node-2: committed diff did not carry before to after");
}
