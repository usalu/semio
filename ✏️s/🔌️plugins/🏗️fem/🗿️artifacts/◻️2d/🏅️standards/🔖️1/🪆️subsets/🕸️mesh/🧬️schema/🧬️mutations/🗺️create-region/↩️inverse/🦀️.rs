//! ↩️ Inverse for `CreateRegion` — always a `delete-region` of the created id.
use super::CreateRegion;
use crate::mutations::{delete_region, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateRegion, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteRegion(delete_region::DeleteRegion { id: payload.region.id.clone() })]
}
//#endregion 🔖️Inverse
