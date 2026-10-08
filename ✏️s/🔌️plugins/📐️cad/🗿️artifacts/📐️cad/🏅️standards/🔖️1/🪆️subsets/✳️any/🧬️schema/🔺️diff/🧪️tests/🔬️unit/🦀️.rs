use super::*;
use crate::mutations::create_node::CreateNode;
use crate::mutations::delete_node::DeleteNode;
use crate::mutations::rename_node::RenameNode;
use crate::op::CadMutation;
use crate::sample_scene_fixture::sample_scene;
use protocol::{DiffAlgebra, Mutation};

fn row(id: &str, label: &str) -> CadNode {
    CadNode { id: id.into(), label: label.into(), kind: "group".into() }
}

#[semio_framework_async_macros::async_test]
async fn node_collection_diffs_absorb_into_one_apply() {
    let base = sample_scene();
    let mut diff = CadMutation::CreateNode(CreateNode { node: CadNode { id: "node-9".into(), label: "Fresh".into(), kind: "group".into() } , index: None }).diff(&base).diff().clone();
    diff.absorb(CadMutation::RenameNode(RenameNode { node_id: "node-1".into(), new_label: "Renamed".into() }).diff(&base).diff().clone());
    let next = protocol::apply_diff(&diff, &base).expect("valid mutation diff");
    assert!(next.nodes.iter().any(|node| node.id == "node-9"));
    assert_eq!(next.nodes.iter().find(|node| node.id == "node-1").expect("node-1").label, "Renamed");
}

#[semio_framework_async_macros::async_test]
async fn malformed_named_diff_rejects_without_changing_the_base() {
    let base = sample_scene();
    let original = base.clone();
    let diff = CadDiff { nodes: Some(CadNodesDelta { removed: vec!["missing-node".into()], ..Default::default() }), ..Default::default() };
    let error = protocol::apply_diff(&diff, &base).expect_err("missing named target must reject");
    assert_eq!(error.code, "mutation.apply.missing-target");
    assert_eq!(error.target, ["nodes", "removed", "0"]);
    assert_eq!(base, original);
}

/// ➕️ create∘delete of the same node cancels, patch∘create folds into the added row, and the rest keeps its order.
#[semio_framework_async_macros::async_test]
async fn absorb_coalesces_same_key_rows() {
    let created = CadDiff { nodes: Some(CadNodesDelta { added: vec![row("fresh", "Fresh"), row("keep", "Keep")], ..Default::default() }), ..Default::default() };
    let mut sum = created.clone();
    sum.absorb(CadDiff { nodes: Some(CadNodesDelta { patched: vec![CadNodePatchEntry { id: "fresh".into(), patch: CadNodePatch { label: Some("Renamed".into()) } }], ..Default::default() }), ..Default::default() });
    assert_eq!(sum.nodes.as_ref().expect("nodes").added[0].label, "Renamed");
    sum.absorb(CadDiff { nodes: Some(CadNodesDelta { removed: vec!["fresh".into()], ..Default::default() }), ..Default::default() });
    let nodes = sum.nodes.as_ref().expect("nodes");
    assert_eq!(nodes.added, vec![row("keep", "Keep")]);
    assert!(nodes.removed.is_empty() && nodes.patched.is_empty());
    let mut cancelled = created;
    cancelled.absorb(CadDiff { nodes: Some(CadNodesDelta { removed: vec!["fresh".into(), "keep".into()], ..Default::default() }), ..Default::default() });
    assert!(cancelled.nodes.is_none(), "create∘delete leaves nothing");
}

/// ➕️ delete∘create of one id keeps the removal and appends the replacement.
#[semio_framework_async_macros::async_test]
async fn absorb_turns_delete_then_create_into_a_replacement() {
    let base = sample_scene();
    let id = base.nodes[0].id.clone();
    let mut sum = CadDiff { nodes: Some(CadNodesDelta { removed: vec![id.clone()], ..Default::default() }), ..Default::default() };
    sum.absorb(CadDiff { nodes: Some(CadNodesDelta { added: vec![row(&id, "Again")], ..Default::default() }), ..Default::default() });
    let next = protocol::apply_diff(&sum, &base).expect("replacement applies");
    assert_eq!(next.nodes.last().map(|node| node.label.as_str()), Some("Again"));
}

/// ↩️ The negative diff restores a removed node at its base position and `between` is the state delta.
#[semio_framework_async_macros::async_test]
async fn inverse_and_between_restore_the_base() {
    let base = sample_scene();
    let removed = CadMutation::DeleteNode(DeleteNode { node_id: base.nodes[0].id.clone() }).diff(&base).diff().clone();
    let after = protocol::apply_diff(&removed, &base).expect("delete applies");
    let restored = protocol::apply_diff(&removed.inverse(&base), &after).expect("negative diff applies");
    assert_eq!(restored, base);
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<CadSnapshot, CadDiff>(&base, &after).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<CadSnapshot, CadDiff>(&after, &base).await;
}
