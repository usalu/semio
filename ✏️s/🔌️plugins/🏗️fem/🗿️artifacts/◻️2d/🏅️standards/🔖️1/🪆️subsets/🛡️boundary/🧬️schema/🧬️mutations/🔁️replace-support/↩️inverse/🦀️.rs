//! ↩️ Inverse for `ReplaceSupport` — recovers the pre-mutation support from `base`.
use super::ReplaceSupport;
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceSupport, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.supports.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::ReplaceSupport(ReplaceSupport { id: payload.id.clone(), new_support: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
