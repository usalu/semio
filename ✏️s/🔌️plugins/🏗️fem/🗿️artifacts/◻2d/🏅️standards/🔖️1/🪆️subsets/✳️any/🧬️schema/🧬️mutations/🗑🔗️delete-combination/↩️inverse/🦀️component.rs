//! ↩️ Inverse for `DeleteCombination` — recreates the captured combination from `base`.
use super::mutation::DeleteCombination;
use crate::artifacts::fem2d::mutations::{create_combination, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteCombination, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.combinations.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::CreateCombination(create_combination::mutation::CreateCombination { combination: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
