//! 🔺️ Sparse diff builder for `ReplaceSupport`.
use super::ReplaceSupport;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSupportsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSupportsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_mismatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSupport, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.supports.iter().find(|support| support.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_support.id != payload.id {
        return target_mismatch("Support", &payload.id, &payload.new_support.id);
    }
    if !semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemDof::canonical(&payload.new_support.fixed) { return protocol::MutationOutcome::fatal("mutation.invariant", "Support restraints must be canonical.", [payload.id.clone()]); }
    if existing == &payload.new_support {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Support \"{}\" already has that value.", payload.id));
    }
    if !base.nodes.iter().any(|node| node.id == payload.new_support.node_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.new_support.node_id), [payload.new_support.node_id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { modified: vec![Fem3dSupportsModification { id: payload.id.clone(), patch: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSupportPatch::replacement(&payload.new_support) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
