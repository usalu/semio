//! ↩️ Inverse for `DeleteMaterial` — recreates the captured material from `base`.
use super::DeleteMaterial;
use crate::standards::v1::subsets::any::schema::mutations::create_material;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<create_material::CreateMaterial>>(payload: &DeleteMaterial, base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.materials.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Op::from(create_material::CreateMaterial { material: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
