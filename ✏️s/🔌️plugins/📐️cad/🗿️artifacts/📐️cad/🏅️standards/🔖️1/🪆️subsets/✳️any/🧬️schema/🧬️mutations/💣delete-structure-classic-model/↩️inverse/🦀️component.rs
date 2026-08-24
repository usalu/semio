//! ↩️ `delete-structure-classic-model` — undo is `create-structure-classic-model` with the escrowed handle captured from BASE;
//! empty (`Vec::new()`) when the slot was already absent (nothing to undo).

use super::mutation::DeleteStructureClassicModel;
use crate::artifacts::cad::mutations::{create_structure_classic_model, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &DeleteStructureClassicModel, base: &CadSnapshot) -> Vec<CadMutation> {
    match &base.structure_classic_model {
        Some(existing) => vec![CadMutation::CreateStructureClassicModel(create_structure_classic_model::mutation::CreateStructureClassicModel { child_id: existing.child_id.clone(), target: existing.target.to_uri() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
