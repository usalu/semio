//! 🔺️ Sparse diff builder for `ChangeLoadCaseName`.
use super::ChangeLoadCaseName;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCasePatch;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCasesDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCasesModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadCaseName, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Load case \"{}\" is already named \"{}\".", payload.case_id, payload.new_name));
    }
    let patch = Fem2dLoadCasePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { modified: vec![Fem2dLoadCasesModification { id: payload.case_id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
