//! ↩️ Inverse for `ChangeLoadCaseName` — recovers the pre-mutation name from `base`.
use super::ChangeLoadCaseName;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::change_load_case_name::ChangeLoadCaseName>>(payload: &ChangeLoadCaseName, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.load_cases.iter().find(|case| case.id == payload.case_id).map(|case| vec![Op::from(ChangeLoadCaseName { case_id: payload.case_id.clone(), new_name: case.name.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
