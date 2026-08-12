//! ↩️ Inverse for `DeleteRegion` — recreates the captured region from `base`.
use super::mutation::DeleteRegion;
use crate::artifacts::fem2d::mutations::{create_region, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteRegion, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.regions
        .iter()
        .find(|item| item.id == payload.id)
        .map(|item| vec![Fem2dMutation::CreateRegion(create_region::mutation::CreateRegion { region: item.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
