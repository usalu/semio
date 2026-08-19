//! ↩️ Inverse for `CreateSolid` — always a `delete-solid` of the created id.
use super::mutation::CreateSolid;
use crate::artifacts::fem3d::mutations::{delete_solid, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateSolid, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteSolid(delete_solid::mutation::DeleteSolid { id: payload.solid.id.clone() })]
}
//#endregion 🔖️Inverse
