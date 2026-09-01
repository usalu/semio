//! ↩️ Inverse for `CreateMaterial` — always a `delete-material` of the created id.
use super::CreateMaterial;
use crate::artifacts::fem2d::mutations::{delete_material, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateMaterial, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: payload.material.id.clone() })]
}
//#endregion 🔖️Inverse
