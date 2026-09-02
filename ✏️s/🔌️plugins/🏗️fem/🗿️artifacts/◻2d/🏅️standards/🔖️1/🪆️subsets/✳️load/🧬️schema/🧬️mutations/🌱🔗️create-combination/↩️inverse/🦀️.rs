//! ↩️ Inverse for `CreateCombination` — always a `delete-combination` of the created id.
use super::CreateCombination;
use crate::artifacts::fem2d::mutations::{delete_combination, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateCombination, _base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    vec![Fem2dMutation::DeleteCombination(delete_combination::DeleteCombination { id: payload.combination.id.clone() })]
}
//#endregion 🔖️Inverse
