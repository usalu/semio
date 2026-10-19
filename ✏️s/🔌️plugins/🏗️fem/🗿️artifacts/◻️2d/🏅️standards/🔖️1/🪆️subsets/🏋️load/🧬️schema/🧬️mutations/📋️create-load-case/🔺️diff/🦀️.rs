//! 🔺️ Sparse diff builder for `CreateLoadCase`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::load_reference` resolution of every carried load's own target, in payload order
//! (`mutation.target-missing`, Error). `add-load` calls the SAME guard, so the two doors into a
//! case's `loads` cannot drift apart.
use super::CreateLoadCase;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCaseInsertion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCasesDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &CreateLoadCase, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    if let Some(rejection) = guards::scoped_identity(&["loadCase", &payload.load_case.id]) { return rejection; }
    if base.load_cases.iter().any(|case| case.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A load case with id \"{}\" already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    let mut load_ids = std::collections::HashSet::new();
    for load in &payload.load_case.loads {
        let id = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id(load);
        if let Some(rejection) = guards::scoped_identity(&["load", &payload.load_case.id, id]).or_else(|| guards::load_magnitudes(load)) { return rejection; }
        if !load_ids.insert(id) { return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A load case carries a repeated load identity.", [id.to_string()]); }
        if let Some(rejection) = guards::load_reference(base, load) {
            return rejection;
        }
    }
    if payload.index.is_some_and(|at| at > base.load_cases.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.load_cases.len()), [&payload.load_case.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { inserted: vec![Fem2dLoadCaseInsertion { index: payload.index.unwrap_or(base.load_cases.len()), row: payload.load_case.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
