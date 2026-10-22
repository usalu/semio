extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_os_kernel as protocol;
use semio_s_artifact_cad_cad_model_diff::*;
use semio_s_artifact_cad_cad_model::{snapshot::CadBrepChild, snapshot::CadDrawingChild, snapshot::CadModelChild, snapshot::CadNode, snapshot::CadReference};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use protocol::list_delta::RowPatch;
use protocol::MutationDiff;
use semio_s_artifact_cad_cad_mut_nodes::create_node::CreateNode;
use semio_s_artifact_cad_cad_mut_nodes::delete_node::DeleteNode;
use semio_s_artifact_cad_cad_mut_nodes::rename_node::RenameNode;
use semio_s_artifact_cad_cad_mutations::CadMutation;
use semio_s_artifact_cad_cad_testing::sample_scene;
use protocol::{DiffAlgebra, ColdMutation, Mutation};

fn row(id: &str, label: &str) -> CadNode {
    CadNode { id: id.into(), label: label.into(), kind: "group".into() }
}

#[semio_framework_async_macros::async_test]
async fn node_collection_diffs_absorb_into_one_apply() {
    let base = sample_scene();
    let mut diff = CadMutation::CreateNode(CreateNode { node: CadNode { id: "node-9".into(), label: "Fresh".into(), kind: "group".into() } , index: None }).diff(&base).diff().clone();
    diff.absorb(CadMutation::RenameNode(RenameNode { node_id: "node-1".into(), new_label: "Renamed".into() }).diff(&base).diff().clone());
    let next = protocol::apply_diff_cold(&diff, &base).expect("valid mutation diff");
    assert!(next.nodes.iter().any(|node| node.id == "node-9"));
    assert_eq!(next.nodes.iter().find(|node| node.id == "node-1").expect("node-1").label, "Renamed");
}

#[semio_framework_async_macros::async_test]
async fn malformed_named_diff_rejects_without_changing_the_base() {
    let base = sample_scene();
    let original = base.clone();
    let diff = CadDiff { nodes: Some(CadNodesDelta { removed: vec![CadNodeRemoval { id: "missing-node".into(), index: 0 }], ..Default::default() }), ..Default::default() };
    let error = protocol::apply_diff_cold(&diff, &base).expect_err("missing named target must reject");
    assert_eq!(error.code, "mutation.apply.missing-target");
    assert_eq!(error.target, ["nodes", "removed", "0"]);
    assert_eq!(base, original);
}

/// ➕️ create∘delete of the same node cancels, patch∘create stays its own modification, and the rest keeps its order.
#[semio_framework_async_macros::async_test]
async fn absorb_coalesces_same_key_rows() {
    let created = CadDiff { nodes: Some(CadNodesDelta { inserted: vec![CadNodeInsertion { index: 0, row: row("fresh", "Fresh") }, CadNodeInsertion { index: 1, row: row("keep", "Keep") }], ..Default::default() }), ..Default::default() };
    let mut sum = created.clone();
    sum.absorb(CadDiff { nodes: Some(CadNodesDelta::modification("fresh", CadNodePatch { label: Some("Renamed".into()) })), ..Default::default() });
    assert_eq!(sum.nodes.as_ref().expect("nodes").inserted[0].row.label, "Fresh", "the framework keeps a patch of an inserted row as its own modified entry");
    assert_eq!(sum.nodes.as_ref().expect("nodes").modified.len(), 1);
    assert_eq!(protocol::apply_diff_cold(&sum, &sample_scene()).expect("composed diff applies").nodes[0].label, "Renamed", "inserted then patched applies in order");
    sum.absorb(CadDiff { nodes: Some(CadNodesDelta { removed: vec![CadNodeRemoval { id: "fresh".into(), index: 0 }], ..Default::default() }), ..Default::default() });
    let nodes = sum.nodes.as_ref().expect("nodes");
    assert_eq!(nodes.inserted.iter().map(|insertion| (insertion.index, insertion.row.clone())).collect::<Vec<_>>(), vec![(0, row("keep", "Keep"))]);
    assert!(nodes.removed.is_empty() && nodes.modified.is_empty());
    let mut cancelled = created;
    cancelled.absorb(CadDiff { nodes: Some(CadNodesDelta { removed: vec![CadNodeRemoval { id: "fresh".into(), index: 0 }, CadNodeRemoval { id: "keep".into(), index: 1 }], ..Default::default() }), ..Default::default() });
    assert!(cancelled.nodes.is_none(), "create∘delete leaves nothing");
}

/// ➕️ remove∘insert of one id keeps the removal and the insertion at its slot: a replacement.
#[semio_framework_async_macros::async_test]
async fn absorb_turns_delete_then_create_into_a_replacement() {
    let base = sample_scene();
    let id = base.nodes[0].id.clone();
    let mut sum = CadDiff { nodes: Some(CadNodesDelta::removal(&base.nodes, 0)), ..Default::default() };
    sum.absorb(CadDiff { nodes: Some(CadNodesDelta::insertion(0, row(&id, "Again"))), ..Default::default() });
    let next = protocol::apply_diff_cold(&sum, &base).expect("replacement applies");
    assert_eq!(next.nodes.first().map(|node| node.label.as_str()), Some("Again"));
}

