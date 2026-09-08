//! ↩️ Inverse for `DeleteCombination` — recreates the captured combination from `base`.
use super::DeleteCombination;
use crate::mutations::{create_combination, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteCombination, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.combinations.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::CreateCombination(create_combination::CreateCombination { combination: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
