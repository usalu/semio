//! ↩️ Inverse for `DeleteMaterial` — recreates the captured material from `base`.
use super::DeleteMaterial;
use crate::mutations::{create_material, Fem3dMutation};
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMaterial, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.materials.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::CreateMaterial(create_material::CreateMaterial { material: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
