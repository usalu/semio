//! ↩️ Inverse for `ReplaceCombination` — recovers the pre-mutation combination from `base`.
use super::ReplaceCombination;
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceCombination, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.combinations
        .iter()
        .find(|item| item.id == payload.id)
        .map(|item| vec![Fem2dMutation::ReplaceCombination(ReplaceCombination { id: payload.id.clone(), new_combination: item.clone() })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
