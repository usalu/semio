//! ↩️ Inverse for `DeleteGeneration`, reconstructed from BASE.
use super::mutation::DeleteGeneration;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use crate::artifacts::procedural2d::mutations::{widget_index};

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteGeneration, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
                Some(entry) => vec![crate::artifacts::procedural2d::mutations::create_generation::mutation::create_generation(entry.clone())],
                None => Vec::new(),
            }
}
//#endregion 🔖️Inverse
