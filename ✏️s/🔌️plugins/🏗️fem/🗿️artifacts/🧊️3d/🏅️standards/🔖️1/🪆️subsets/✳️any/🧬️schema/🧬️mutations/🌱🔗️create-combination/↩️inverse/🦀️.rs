//! ↩️ Inverse for `CreateCombination` — always a `delete-combination` of the created id.
use super::CreateCombination;
use crate::artifacts::fem3d::mutations::{delete_combination, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateCombination, _base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    vec![Fem3dMutation::DeleteCombination(delete_combination::DeleteCombination { id: payload.combination.id.clone() })]
}
//#endregion 🔖️Inverse
