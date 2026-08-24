//! ↩️ `delete-building-model` — undo is `create-building-model` with the escrowed handle captured from BASE;
//! empty (`Vec::new()`) when the slot was already absent (nothing to undo).

use super::mutation::DeleteBuildingModel;
use crate::artifacts::cad::mutations::{create_building_model, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &DeleteBuildingModel, base: &CadSnapshot) -> Vec<CadMutation> {
    match &base.building_model {
        Some(existing) => vec![CadMutation::CreateBuildingModel(create_building_model::mutation::CreateBuildingModel { child_id: existing.child_id.clone(), target: existing.target.to_uri() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
