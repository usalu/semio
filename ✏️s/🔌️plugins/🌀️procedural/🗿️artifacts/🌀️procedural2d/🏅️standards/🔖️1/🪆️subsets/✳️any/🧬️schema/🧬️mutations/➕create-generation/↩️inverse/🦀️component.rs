//! ↩️ Inverse for `CreateGeneration`, reconstructed from BASE.
use super::mutation::CreateGeneration;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::{widget_id, widget_index, Procedural2dSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &CreateGeneration, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    vec![crate::artifacts::procedural2d::mutations::delete_generation::delete_generation(payload.generation.id.clone())]
}
//#endregion 🔖️Inverse
