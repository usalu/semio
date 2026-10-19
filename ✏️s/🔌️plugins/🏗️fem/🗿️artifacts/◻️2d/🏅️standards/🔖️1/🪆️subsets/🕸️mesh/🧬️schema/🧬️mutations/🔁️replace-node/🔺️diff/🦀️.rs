//! 🔺️ Sparse diff builder for `ReplaceNode`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.target-mismatch` (Error) when the replacement renames it, the SAME `node_geometry` bound
//! `create-node` runs (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceNode;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodePatch;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodesDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodesModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceNode, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("node", &payload.id, &payload.new_node.id) {
        return rejection;
    }
    if let Some(rejection) = guards::node_geometry(&payload.new_node) {
        return rejection;
    }
    if *existing == payload.new_node {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { nodes: Some(Fem2dNodesDelta { modified: vec![Fem2dNodesModification { id: payload.id.clone(), patch: Fem2dNodePatch::replacement(&payload.new_node) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
