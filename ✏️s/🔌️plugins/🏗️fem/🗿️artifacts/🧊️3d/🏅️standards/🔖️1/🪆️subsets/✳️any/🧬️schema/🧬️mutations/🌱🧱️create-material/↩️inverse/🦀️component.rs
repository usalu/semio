//! ↩️ Inverse for `CreateMaterial` — always a `delete-material` of the created id.
use super::mutation::CreateMaterial;
use crate::artifacts::fem3d::mutations::{delete_material, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateMaterial, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: payload.material.id.clone() })]
}
//#endregion 🔖️Inverse
