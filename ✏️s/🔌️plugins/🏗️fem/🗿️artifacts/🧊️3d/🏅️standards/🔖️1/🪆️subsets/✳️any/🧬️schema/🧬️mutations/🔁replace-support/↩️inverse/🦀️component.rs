//! ↩️ Inverse for `ReplaceSupport` — recovers the pre-mutation support from `base`.
use super::mutation::ReplaceSupport;
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceSupport, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.supports
        .iter()
        .find(|item| item.id == payload.id)
        .map(|item| vec![Fem3dMutation::ReplaceSupport(ReplaceSupport { id: payload.id.clone(), new_support: item.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
