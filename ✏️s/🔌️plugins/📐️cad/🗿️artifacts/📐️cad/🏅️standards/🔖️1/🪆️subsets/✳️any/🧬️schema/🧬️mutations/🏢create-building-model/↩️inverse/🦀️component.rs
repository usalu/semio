//! ↩️ `create-building-model` — undo restores whichever handle occupied `building_model` BEFORE this create ran
//! (a real prior handle if the slot was occupied, or `delete-building-model` if it was empty) — never a
//! bare "delete", since `create-building-model` may have OVERWRITTEN an existing handle.

use super::mutation::CreateBuildingModel;
use crate::artifacts::cad::mutations::{delete_building_model, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &CreateBuildingModel, base: &CadSnapshot) -> Vec<CadMutation> {
    match &base.building_model {
        Some(existing) => vec![CadMutation::CreateBuildingModel(CreateBuildingModel { child_id: existing.child_id.clone(), target: existing.target.to_uri() })],
        None => vec![CadMutation::DeleteBuildingModel(delete_building_model::mutation::DeleteBuildingModel {})],
    }
}
//#endregion 🔖️Inverse
