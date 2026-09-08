//! ↩️ Inverse for `DeleteCombination` — recreates the captured combination from `base`.
use super::DeleteCombination;
use crate::standards::v1::subsets::any::schema::mutations::{create_combination, Fem3dMutation};
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteCombination, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.combinations.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::CreateCombination(create_combination::CreateCombination { combination: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
