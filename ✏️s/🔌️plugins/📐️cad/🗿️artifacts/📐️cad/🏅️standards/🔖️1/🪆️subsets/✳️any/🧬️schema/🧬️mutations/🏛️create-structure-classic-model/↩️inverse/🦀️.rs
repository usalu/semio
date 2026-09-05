//! ↩️ `create-structure-classic-model` — undo restores whichever handle occupied `structure_classic_model` BEFORE this create ran
//! (a real prior handle if the slot was occupied, or `delete-structure-classic-model` if it was empty) — never a
//! bare "delete", since `create-structure-classic-model` may have OVERWRITTEN an existing handle.

use super::CreateStructureClassicModel;
use crate::artifacts::cad::mutations::{delete_structure_classic_model, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &CreateStructureClassicModel, base: &CadSnapshot) -> Vec<CadMutation> {
    match &base.structure_classic_model {
        Some(existing) => vec![CadMutation::CreateStructureClassicModel(CreateStructureClassicModel { child_id: existing.child_id.clone(), target: existing.target.to_uri() })],
        None => vec![CadMutation::DeleteStructureClassicModel(delete_structure_classic_model::DeleteStructureClassicModel {})],
    }
}
//#endregion 🔖️Inverse
