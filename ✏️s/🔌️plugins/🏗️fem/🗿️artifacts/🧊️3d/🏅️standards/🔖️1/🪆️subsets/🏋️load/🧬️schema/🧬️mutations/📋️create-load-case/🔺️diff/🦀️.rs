//! 🔺️ Sparse diff builder for `CreateLoadCase`.
use super::CreateLoadCase;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCaseInsertion;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasesDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::resolve_load;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &CreateLoadCase, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    if let Some(message) = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::entity_identity_breach("loadCase", &payload.load_case.id) { return protocol::MutationOutcome::fatal("mutation.invariant", message, [payload.load_case.id.clone()]); }
    if base.load_cases.iter().any(|case| case.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A load case with id \"{}\" already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    let mut seen = std::collections::HashSet::new();
    for load in &payload.load_case.loads {
        let id = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id(load);
        if protocol::wire::validate_scoped_identity_segments(&["load", &payload.load_case.id, id]).is_err() || !seen.insert(id) { return protocol::MutationOutcome::fatal("mutation.invariant", "Load identity is repeated in its case or exceeds scoped identity bounds.", [payload.load_case.id.clone(), id.into()]); }
        if let Some(refusal) = resolve_load(base, load) {
            return refusal;
        }
    }
    if payload.index.is_some_and(|at| at > base.load_cases.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.load_cases.len()), [&payload.load_case.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { inserted: vec![Fem3dLoadCaseInsertion { index: payload.index.unwrap_or(base.load_cases.len()), row: payload.load_case.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
