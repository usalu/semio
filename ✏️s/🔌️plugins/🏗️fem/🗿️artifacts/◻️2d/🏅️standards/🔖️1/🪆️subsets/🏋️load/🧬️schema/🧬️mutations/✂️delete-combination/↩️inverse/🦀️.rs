//! ↩️ Inverse for `DeleteCombination` — recreates the captured combination from `base`.
use super::DeleteCombination;
use crate::standards::v1::subsets::any::schema::mutations::create_combination;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::create_combination::CreateCombination>>(payload: &DeleteCombination, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.combinations.iter().enumerate().find(|(_, item)| item.id == payload.id).map(|(at, item)| vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::create_combination::CreateCombination { combination: item.clone(), index: Some(at) })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
