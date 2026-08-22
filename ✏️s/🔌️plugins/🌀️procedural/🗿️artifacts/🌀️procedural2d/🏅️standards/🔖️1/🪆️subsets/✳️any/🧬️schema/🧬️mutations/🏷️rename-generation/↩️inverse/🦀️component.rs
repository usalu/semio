//! ↩️ Inverse for `RenameGeneration`, reconstructed from BASE.
use super::mutation::RenameGeneration;
use crate::artifacts::procedural2d::mutations::rename_generation;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RenameGeneration, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![rename_generation(payload.id.clone(), entry.name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
