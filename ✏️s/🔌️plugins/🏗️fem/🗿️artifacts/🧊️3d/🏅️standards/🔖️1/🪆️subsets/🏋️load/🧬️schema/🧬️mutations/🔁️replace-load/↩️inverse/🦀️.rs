//! ↩️ Inverse for `ReplaceLoad` — recovers the pre-mutation load from `base`.
use super::ReplaceLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<replace_load::ReplaceLoad>>(payload: &ReplaceLoad, base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.load_cases
        .iter()
        .find(|case| case.id == payload.case_id)
        .and_then(|case| case.loads.iter().find(|load| load_id(load) == payload.load_id).cloned())
        .map(|load| vec![Op::from(ReplaceLoad { case_id: payload.case_id.clone(), load_id: payload.load_id.clone(), new_load: Box::new(load) })])
        .unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
