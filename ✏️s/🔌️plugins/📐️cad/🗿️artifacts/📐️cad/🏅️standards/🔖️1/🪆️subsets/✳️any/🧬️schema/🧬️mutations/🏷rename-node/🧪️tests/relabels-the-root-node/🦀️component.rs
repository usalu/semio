//! 🧪️ `rename-node` fixture — `relabels-the-root-node`.
//!
//! Proves the node `label` is patched while `kind` — the only other field `CadNodePatch` could carry — stays put.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> CadSnapshot {
    serde_json::from_str(BEFORE).expect("rename-node/relabels-the-root-node: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("rename-node/relabels-the-root-node: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("rename-node/relabels-the-root-node: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("rename-node applies to its committed before-snapshot")
}

/// ▶️ `rename-node` patches `label` only; `CadNodePatch` has no `kind` field, so the node's type can never drift here.
#[semio_framework_async_macros::async_test]
async fn relabels_the_node_without_retyping_it() {
    let after = applied();
    let node = after.nodes.iter().find(|node| node.id == "node-1").expect("node-1 survives");
    assert_eq!(node.label, "Assembly Root", "rename-node must set the addressed node's label");
    assert_eq!(node.kind, "group", "rename-node must leave the node kind untouched");
    assert_eq!(after.nodes[1].label, "Base Plate", "rename-node must not relabel sibling nodes");
    assert_eq!(after, expected_after(), "rename-node/relabels-the-root-node: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `rename-node` carrying the label captured from BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_root_label() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "rename-node inverts to exactly one step");
    match &inverse[0] {
        CadMutation::RenameNode(step) => {
            assert_eq!(step.node_id, "node-1", "the inverse must address the same node");
            assert_eq!(step.new_label, "Root", "the inverse must carry the pre-edit label");
        }
        other => panic!("rename-node must invert to rename-node, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("rename-node/relabels-the-root-node: inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-node/relabels-the-root-node: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-node/relabels-the-root-node: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-node/relabels-the-root-node: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `rename-node`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-node/relabels-the-root-node: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "rename-node/relabels-the-root-node: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().nodes.as_ref().expect("rename-node fills the nodes delta");
    assert_eq!(delta.patched.len(), 1, "rename-node patches exactly one node");
    assert_eq!(delta.patched[0].id, "node-1", "rename-node's patch entry addresses node-1");
    assert_eq!(delta.patched[0].patch.label.as_deref(), Some("Assembly Root"), "rename-node fills the patch's `label` field — the only field CadNodePatch has");
    assert!(delta.added.is_empty() && delta.removed.is_empty(), "rename-node touches only the `patched` arm of the nodes delta");
}

/// 🔺️ The sparse delta `rename-node` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here `CadNodePatch` has exactly one field, so the diff structurally CANNOT retype the node.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-node/relabels-the-root-node: rename-node must emit a node patch carrying the new label and nothing else");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-node/relabels-the-root-node: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `rename-node` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-node/relabels-the-root-node: committed diff did not carry before to after");
}
