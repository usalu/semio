//! ↩️ Inverse for `ReplaceMaterial` — recovers the pre-mutation material from `base`.
use super::ReplaceMaterial;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<replace_material::ReplaceMaterial>>(payload: &ReplaceMaterial, base: &Fem2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.materials.iter().find(|item| item.id == payload.id).map(|item| vec![Op::from(ReplaceMaterial { id: payload.id.clone(), new_material: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
