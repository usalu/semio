//! 🔺️ Sparse diff builder for `ChangeLoadCaseSelfWeight`.
use super::ChangeLoadCaseSelfWeight;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasePatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasesDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasesModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadCaseSelfWeight, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    if existing.self_weight == payload.new_self_weight {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Load case \"{}\" already has self-weight {}.", payload.case_id, payload.new_self_weight));
    }
    let patch = Fem3dLoadCasePatch { self_weight: Some(payload.new_self_weight), ..Default::default() };
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { modified: vec![Fem3dLoadCasesModification { id: payload.case_id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
