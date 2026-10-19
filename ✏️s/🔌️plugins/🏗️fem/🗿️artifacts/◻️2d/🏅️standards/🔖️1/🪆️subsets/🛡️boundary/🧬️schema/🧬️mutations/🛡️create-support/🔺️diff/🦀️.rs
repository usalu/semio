//! 🔺️ Sparse diff builder for `CreateSupport`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::node_reference` resolution of `node_id` (`mutation.target-missing`, Error).
//! `replace-support` calls the SAME guard, so the twins cannot drift apart.
use super::CreateSupport;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSupportInsertion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSupportsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &CreateSupport, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    if let Some(rejection) = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards::scoped_identity(&["support", &payload.support.id]) { return rejection; }
    if base.supports.iter().any(|support| support.id == payload.support.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A support with id \"{}\" already exists.", payload.support.id), [payload.support.id.clone()]);
    }
    if let Some(rejection) = guards::node_reference(base, &payload.support.node_id) {
        return rejection;
    }
    if !semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::canonical(&payload.support.fixed) { return protocol::MutationOutcome::fatal("mutation.invariant", "Support restraints must be canonical.", [payload.support.id.clone()]); }
    if payload.index.is_some_and(|at| at > base.supports.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.supports.len()), [&payload.support.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { supports: Some(Fem2dSupportsDelta { inserted: vec![Fem2dSupportInsertion { index: payload.index.unwrap_or(base.supports.len()), row: payload.support.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
