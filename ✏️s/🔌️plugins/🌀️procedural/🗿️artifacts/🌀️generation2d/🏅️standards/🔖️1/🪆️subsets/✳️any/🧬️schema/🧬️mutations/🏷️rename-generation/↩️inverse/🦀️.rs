//! ↩️ Inverse for `RenameGeneration`, reconstructed from BASE.
use super::RenameGeneration;
use crate::artifacts::generation2d::mutations::rename_generation;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RenameGeneration, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![rename_generation(payload.id.clone(), entry.name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
