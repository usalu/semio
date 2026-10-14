//! 🔺️ Sparse diff builder for `ReplaceNode`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.target-mismatch` (Error) when the replacement renames it, the SAME finite-position bound
//! `create-node` runs (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceNode;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodesDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dNodesModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_mismatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::invariant;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::node_breach;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceNode, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_node.id != payload.id {
        return target_mismatch("Node", &payload.id, &payload.new_node.id);
    }
    if let Some(breach) = node_breach(&payload.new_node) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    if *existing == payload.new_node {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" already has that value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { nodes: Some(Fem3dNodesDelta { modified: vec![Fem3dNodesModification { id: payload.id.clone(), patch: payload.new_node.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
