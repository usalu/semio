//! ↩️ Inverse for `DeleteRegion` — recreates the captured region from `base`.
use super::DeleteRegion;
use crate::standards::v1::subsets::any::schema::mutations::{create_region, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteRegion, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.regions.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::CreateRegion(create_region::CreateRegion { region: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
