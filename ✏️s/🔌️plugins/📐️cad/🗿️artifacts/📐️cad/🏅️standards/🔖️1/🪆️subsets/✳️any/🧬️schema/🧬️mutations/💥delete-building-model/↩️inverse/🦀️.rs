//! ↩️ `delete-building-model` — undo is `create-building-model` with the escrowed handle captured from BASE;
//! empty (`Vec::new()`) when the slot was already absent (nothing to undo).

use super::DeleteBuildingModel;
use crate::create_building_model;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_building_model::CreateBuildingModel>>(_payload: &DeleteBuildingModel, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match &base.building_model {
        Some(existing) => vec![Op::from(create_building_model::CreateBuildingModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
