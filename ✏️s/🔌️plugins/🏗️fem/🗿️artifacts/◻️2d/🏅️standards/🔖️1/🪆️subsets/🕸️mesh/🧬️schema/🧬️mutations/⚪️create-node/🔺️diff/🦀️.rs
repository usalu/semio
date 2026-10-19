//! 🔺️ Sparse diff builder for `CreateNode`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::node_geometry` finiteness bound (`mutation.invariant`, Fatal).
use super::CreateNode;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodeInsertion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodesDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    if let Some(rejection) = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards::scoped_identity(&["node", &payload.node.id]) { return rejection; }
    if base.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    if let Some(rejection) = guards::node_geometry(&payload.node) {
        return rejection;
    }
    if payload.index.is_some_and(|at| at > base.nodes.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.nodes.len()), [&payload.node.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { nodes: Some(Fem2dNodesDelta { inserted: vec![Fem2dNodeInsertion { index: payload.index.unwrap_or(base.nodes.len()), row: payload.node.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
