//! 🔺️ Sparse diff builder for `ChangeLoadCaseName`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on `case_id`, then
//! `mutation.no-op` (Warning) for an unchanged name — no `Fatal` branch at all.
use super::ChangeLoadCaseName;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasePatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasesDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasesModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadCaseName, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Load case \"{}\" is already named \"{}\".", payload.case_id, payload.new_name));
    }
    let patch = Fem3dLoadCasePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { modified: vec![Fem3dLoadCasesModification { id: payload.case_id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
