//! ↩️ Inverse for `DeleteCombination` — recreates the captured combination from `base`.
use super::DeleteCombination;
use crate::standards::v1::subsets::any::schema::mutations::{create_combination,Fem3dMutation};

use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteCombination, base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.combinations.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Fem3dMutation::CreateCombination(create_combination::CreateCombination { combination: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
