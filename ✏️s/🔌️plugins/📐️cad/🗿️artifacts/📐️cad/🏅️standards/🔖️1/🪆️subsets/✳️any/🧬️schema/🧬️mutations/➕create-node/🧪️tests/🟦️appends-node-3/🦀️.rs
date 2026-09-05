//! 🧪️ `create-node` fixture — `🟦️appends-node-3`.
//!
//! Proves a whole `CadNode` record (label and kind) enters the id-keyed node collection.
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
    serde_json::from_str(BEFORE).expect("create-node/appends-node-3: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("create-node/appends-node-3: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("create-node/appends-node-3: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("create-node applies to its committed before-snapshot")
}

/// ▶️ `create-node` appends the payload's `CadNode`, label and kind carried verbatim.
#[semio_framework_async_macros::async_test]
async fn brings_a_whole_node_record_into_the_tree() {
    let after = applied();
    assert_eq!(after.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), vec!["node-1", "node-2", "node-3"], "create-node appends the new node (the nodes delta's `added` always pushes at the end)");
    let created = after.nodes.iter().find(|node| node.id == "node-3").expect("create-node inserts node-3");
    assert_eq!(created.label, "Column", "create-node must carry the payload node's label");
    assert_eq!(created.kind, "solid", "create-node must carry the payload node's kind");
    assert_eq!(after.references_by_model_definition_id["spatial.shape"].len(), 1, "create-node must not touch the reference lists");
    assert_eq!(after, expected_after(), "create-node/appends-node-3: applied state differs from the committed after-snapshot");
}

/// ↩️ `create-node` always inverts to `delete-node` of the id it minted — it never inspects BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_node_it_created() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "create-node inverts to exactly one step");
    match &inverse[0] {
        CadMutation::DeleteNode(step) => assert_eq!(step.node_id, "node-3", "the inverse must delete the node id create-node minted"),
        other => panic!("create-node must invert to delete-node, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("create-node/appends-node-3: inverse step applies");
    }
    assert_eq!(snapshot, base, "create-node/appends-node-3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-node/appends-node-3: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-node/appends-node-3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-node`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-node/appends-node-3: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "create-node/appends-node-3: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().nodes.as_ref().expect("create-node fills the nodes delta");
    assert_eq!(delta.added.len(), 1, "create-node adds exactly one node");
    assert_eq!(delta.added[0].id, "node-3", "create-node's `added` entry is the payload node");
    assert!(delta.removed.is_empty() && delta.patched.is_empty() && delta.reordered.is_none(), "create-node touches only the `added` arm of the nodes delta");
}

/// 🔺️ The sparse delta `create-node` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `nodes.added` is populated — the node tree IS an added/removed/patched delta, unlike `drawings`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-node/appends-node-3: create-node must emit a nodes delta whose only populated arm is `added`");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-node/appends-node-3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `create-node` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-node/appends-node-3: committed diff did not carry before to after");
}
