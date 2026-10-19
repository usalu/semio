//! 🔺️ Sparse diff builder for `CreateNode`.
use super::CreateNode;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodeInsertion;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodesDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::invariant;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::node_breach;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    if let Some(breach) = node_breach(&payload.node) {
        return invariant(breach, vec![payload.node.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.nodes.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.nodes.len()), [&payload.node.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { nodes: Some(Fem3dNodesDelta { inserted: vec![Fem3dNodeInsertion { index: payload.index.unwrap_or(base.nodes.len()), row: payload.node.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
