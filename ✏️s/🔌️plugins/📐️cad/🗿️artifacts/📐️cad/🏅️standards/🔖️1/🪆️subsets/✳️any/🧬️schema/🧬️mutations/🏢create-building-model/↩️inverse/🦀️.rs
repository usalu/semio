//! ↩️ `create-building-model` — undo restores whichever handle occupied `building_model` BEFORE this create ran
//! (a real prior handle if the slot was occupied, or `delete-building-model` if it was empty) — never a
//! bare "delete", since `create-building-model` may have OVERWRITTEN an existing handle.

use super::CreateBuildingModel;
use crate::delete_building_model;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_building_model::CreateBuildingModel> + From<crate::delete_building_model::DeleteBuildingModel>>(_payload: &CreateBuildingModel, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match &base.building_model {
        Some(existing) => vec![Op::from(CreateBuildingModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => vec![Op::from(delete_building_model::DeleteBuildingModel {})],
    })
}
//#endregion 🔖️Inverse
