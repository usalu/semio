//! ↩️ Inverse for `ChangeLoadCaseSelfWeight` — recovers the pre-mutation flag from `base`.
use super::ChangeLoadCaseSelfWeight;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight::ChangeLoadCaseSelfWeight>>(payload: &ChangeLoadCaseSelfWeight, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.load_cases.iter().find(|case| case.id == payload.case_id).map(|case| vec![Op::from(ChangeLoadCaseSelfWeight { case_id: payload.case_id.clone(), new_self_weight: case.self_weight })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
