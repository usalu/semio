//! ↩️ Inverse for `ReplaceSolid` — recovers the pre-mutation solid from `base`.
use super::mutation::ReplaceSolid;
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceSolid, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.solids
        .iter()
        .find(|item| item.id == payload.id)
        .map(|item| vec![Fem3dMutation::ReplaceSolid(ReplaceSolid { id: payload.id.clone(), new_solid: item.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
