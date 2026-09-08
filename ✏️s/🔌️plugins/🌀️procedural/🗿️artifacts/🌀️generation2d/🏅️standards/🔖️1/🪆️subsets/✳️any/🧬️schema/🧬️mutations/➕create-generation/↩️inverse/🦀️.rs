//! ↩️ Inverse for `CreateGeneration`, reconstructed from BASE.
use super::CreateGeneration;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateGeneration, _base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_generation::delete_generation(payload.generation.id.clone())]
}
//#endregion 🔖️Inverse
