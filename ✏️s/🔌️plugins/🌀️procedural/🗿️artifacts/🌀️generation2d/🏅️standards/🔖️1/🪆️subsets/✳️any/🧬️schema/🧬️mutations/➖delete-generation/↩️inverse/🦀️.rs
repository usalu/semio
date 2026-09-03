//! ↩️ Inverse for `DeleteGeneration`, reconstructed from BASE.
use super::DeleteGeneration;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteGeneration, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![crate::artifacts::generation2d::mutations::create_generation::create_generation(entry.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
