//! ↩️ Inverse for `DeleteMaterial` — recreates the captured material from `base`.
use super::mutation::DeleteMaterial;
use crate::artifacts::fem2d::mutations::{create_material, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMaterial, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.materials.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::CreateMaterial(create_material::mutation::CreateMaterial { material: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
