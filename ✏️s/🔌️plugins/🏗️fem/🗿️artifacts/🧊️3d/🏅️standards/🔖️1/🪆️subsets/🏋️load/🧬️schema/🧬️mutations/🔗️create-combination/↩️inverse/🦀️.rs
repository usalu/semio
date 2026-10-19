//! ↩️ Inverse for `CreateCombination` — always a `delete-combination` of the created id.
use super::CreateCombination;
use crate::standards::v1::subsets::any::schema::mutations::delete_combination;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_combination::DeleteCombination>>(payload: &CreateCombination, _base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::delete_combination::DeleteCombination { id: payload.combination.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
