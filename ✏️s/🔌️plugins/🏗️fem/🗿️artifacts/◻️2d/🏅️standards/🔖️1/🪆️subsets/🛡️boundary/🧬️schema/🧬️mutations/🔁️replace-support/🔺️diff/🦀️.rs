//! 🔺️ Sparse diff builder for `ReplaceSupport`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.target-mismatch` (Error) when the replacement renames it, the SAME `node_id` resolution
//! `create-support` runs (`mutation.target-missing`, Error), and finally `mutation.no-op`.
use super::ReplaceSupport;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSupportsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSupportsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSupport, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.supports.iter().find(|support| support.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("support", &payload.id, &payload.new_support.id) {
        return rejection;
    }
    if let Some(rejection) = guards::node_reference(base, &payload.new_support.node_id) {
        return rejection;
    }
    if !crate::FemDof::canonical(&payload.new_support.fixed) { return protocol::MutationOutcome::fatal("mutation.invariant", "Support restraints must be canonical.", [payload.id.clone()]); }
    if *existing == payload.new_support {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Support \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { supports: Some(Fem2dSupportsDelta { modified: vec![Fem2dSupportsModification { id: payload.id.clone(), patch: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSupportPatch::replacement(&payload.new_support) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
