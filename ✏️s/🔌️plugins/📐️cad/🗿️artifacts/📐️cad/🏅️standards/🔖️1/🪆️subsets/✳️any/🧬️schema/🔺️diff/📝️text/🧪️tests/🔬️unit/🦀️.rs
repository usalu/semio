use super::*;
use crate::mutations::create_node::CreateNode;
use crate::mutations::delete_node::DeleteNode;
use crate::mutations::rename_node::RenameNode;
use crate::op::CadMutation;
use crate::testkit::sample_scene;
use protocol::Mutation;

/// ⚖️ `CadDiff.artifact` (a whole-artifact replacement fragment) still exists as a `CadDiff`
/// FIELD — only its former mutation source (`SetSnapshot`, banned per taxonomy) is gone; this
/// law still holds for whatever future non-mutation path (`ArtifactStore::reset`) populates it.
#[semio_framework_async_macros::async_test]
async fn whole_artifact_diff_replaces_the_snapshot_and_absorbs_every_earlier_edit() {
    let base = sample_scene();
    let mut diff = CadMutation::DeleteNode(DeleteNode { node_id: "node-1".into() }).diff(&base).diff().clone();
    let replacement = CadDiff { artifact: Some(Box::new(CadArtifact::from_snapshot(base.clone()))), ..Default::default() };
    diff.absorb(replacement);
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), base, "a whole-artifact diff wins over anything absorbed before it");
}

#[semio_framework_async_macros::async_test]
async fn node_collection_diffs_absorb_into_one_apply() {
    let base = sample_scene();
    let mut diff = CadMutation::CreateNode(CreateNode { node: CadNode { id: "node-9".into(), label: "Fresh".into(), kind: "group".into() } }).diff(&base).diff().clone();
    diff.absorb(CadMutation::RenameNode(RenameNode { node_id: "node-1".into(), new_label: "Renamed".into() }).diff(&base).diff().clone());
    let next = diff.apply(&base).expect("valid mutation diff");
    assert!(next.nodes.iter().any(|node| node.id == "node-9"));
    assert_eq!(next.nodes.iter().find(|node| node.id == "node-1").expect("node-1").label, "Renamed");
}

#[semio_framework_async_macros::async_test]
async fn malformed_named_diff_rejects_without_changing_the_base() {
    let base = sample_scene();
    let original = base.clone();
    let diff = CadDiff { nodes: Some(CadNodesDelta { removed: vec!["missing-node".into()], ..Default::default() }), ..Default::default() };
    let error = diff.apply(&base).expect_err("missing named target must reject");
    assert_eq!(error.code, "mutation.apply.missing-target");
    assert_eq!(error.target, ["nodes", "removed", "0"]);
    assert_eq!(base, original);
}
