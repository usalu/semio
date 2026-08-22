//! ↩️ Inverse for `DeleteMaterial` — recreates the captured material from `base`.
use super::mutation::DeleteMaterial;
use crate::artifacts::fem3d::mutations::{create_material, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMaterial, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.materials.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::CreateMaterial(create_material::mutation::CreateMaterial { material: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
