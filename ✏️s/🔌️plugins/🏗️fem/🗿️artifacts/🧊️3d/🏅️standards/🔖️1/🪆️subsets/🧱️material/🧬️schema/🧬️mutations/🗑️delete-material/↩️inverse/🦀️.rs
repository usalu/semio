//! ↩️ Inverse for `DeleteMaterial` — recreates the captured material from `base`.
use super::DeleteMaterial;
use crate::standards::v1::subsets::any::schema::mutations::{create_material, Fem3dMutation};
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMaterial, base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.materials.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::CreateMaterial(create_material::CreateMaterial { material: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
