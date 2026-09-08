//! ↩️ Inverse for `RenameGeneration`, reconstructed from BASE.
use super::RenameGeneration;
use crate::mutations::rename_generation;
use crate::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RenameGeneration, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![rename_generation(payload.id.clone(), entry.name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
