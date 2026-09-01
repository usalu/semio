//! ↩️ Inverse for `CreateRegion` — always a `delete-region` of the created id.
use super::CreateRegion;
use crate::artifacts::fem2d::mutations::{delete_region, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateRegion, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteRegion(delete_region::DeleteRegion { id: payload.region.id.clone() })]
}
//#endregion 🔖️Inverse
