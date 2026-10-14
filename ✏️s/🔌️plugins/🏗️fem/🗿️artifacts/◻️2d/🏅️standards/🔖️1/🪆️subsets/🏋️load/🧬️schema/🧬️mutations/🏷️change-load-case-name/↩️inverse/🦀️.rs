//! ↩️ Inverse for `ChangeLoadCaseName` — recovers the pre-mutation name from `base`.
use super::ChangeLoadCaseName;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<change_load_case_name::ChangeLoadCaseName>>(payload: &ChangeLoadCaseName, base: &Fem2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.load_cases.iter().find(|case| case.id == payload.case_id).map(|case| vec![Op::from(ChangeLoadCaseName { case_id: payload.case_id.clone(), new_name: case.name.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
