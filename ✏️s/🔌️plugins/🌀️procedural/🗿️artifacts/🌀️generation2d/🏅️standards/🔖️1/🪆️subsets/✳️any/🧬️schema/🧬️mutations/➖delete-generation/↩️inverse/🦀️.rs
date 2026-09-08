//! ↩️ Inverse for `DeleteGeneration`, reconstructed from BASE.
use super::DeleteGeneration;
use crate::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteGeneration, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![crate::mutations::create_generation::create_generation(entry.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
