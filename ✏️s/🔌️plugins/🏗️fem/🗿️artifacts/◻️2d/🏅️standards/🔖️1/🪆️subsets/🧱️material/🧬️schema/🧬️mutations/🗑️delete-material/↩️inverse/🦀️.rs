//! ↩️ Inverse for `DeleteMaterial` — recreates the captured material from `base`.
use super::DeleteMaterial;
use crate::standards::v1::subsets::any::schema::mutations::{create_material, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMaterial, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.materials.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::CreateMaterial(create_material::CreateMaterial { material: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
