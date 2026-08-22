//! ↩️ Inverse for `CreateGeneration`, reconstructed from BASE.
use super::mutation::CreateGeneration;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateGeneration, _base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    vec![crate::artifacts::procedural2d::mutations::delete_generation::mutation::delete_generation(payload.generation.id.clone())]
}
//#endregion 🔖️Inverse
