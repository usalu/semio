//! ↩️ Inverse for `ReplaceMaterial` — recovers the pre-mutation material from `base`.
use super::ReplaceMaterial;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_material::ReplaceMaterial>>(payload: &ReplaceMaterial, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.materials.iter().find(|item| item.id == payload.id).map(|item| vec![Op::from(ReplaceMaterial { id: payload.id.clone(), new_material: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
