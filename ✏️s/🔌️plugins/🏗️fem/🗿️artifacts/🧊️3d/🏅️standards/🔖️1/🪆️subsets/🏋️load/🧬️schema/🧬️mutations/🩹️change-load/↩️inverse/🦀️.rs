//! ↩️ Restores only the addressed load fields in its owning case.
use super::ChangeLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::list_delta::RowPatch;
pub fn inverse<Op: From<ChangeLoad>>(payload: &ChangeLoad, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.load_cases.iter().find(|case| case.id == payload.case_id).and_then(|case| case.loads.iter().find(|row| semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id(row) == payload.id))
        .filter(|row| payload.patch.candidate(row).is_ok_and(|candidate| candidate != **row))
        .map(|row| vec![Op::from(ChangeLoad { case_id: payload.case_id.clone(), id: payload.id.clone(), patch: payload.patch.inverse(row) })]).unwrap_or_default())
}