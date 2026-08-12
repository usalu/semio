//! ↩️ Inverse for `DeleteSolid` — recreates the captured solid from `base`.
use super::mutation::DeleteSolid;
use crate::artifacts::fem3d::mutations::{create_solid, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteSolid, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.solids
        .iter()
        .find(|item| item.id == payload.id)
        .map(|item| vec![Fem3dMutation::CreateSolid(create_solid::mutation::CreateSolid { solid: item.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
