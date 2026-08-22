//! ↩️ Inverse for `ReplaceRegion` — recovers the pre-mutation region from `base`.
use super::mutation::ReplaceRegion;
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceRegion, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.regions.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::ReplaceRegion(ReplaceRegion { id: payload.id.clone(), new_region: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
