//! 🔺️ Sparse diff builder for `DeleteLoadCase`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error), then
//! `mutation.target-referenced` (Error) while any combination still weights this case.
use super::DeleteLoadCase;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCasesDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCaseRemoval;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &DeleteLoadCase, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(at) = base.load_cases.iter().position(|case| case.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::referenced("Load case", "combination term", &payload.id, guards::load_case_referrers(base, &payload.id)) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { removed: vec![Fem2dLoadCaseRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
