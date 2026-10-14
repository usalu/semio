//! ↩️ `create-shape-model` — undo restores whichever handle occupied `shape_model` BEFORE this create ran
//! (a real prior handle if the slot was occupied, or `delete-shape-model` if it was empty) — never a
//! bare "delete", since `create-shape-model` may have OVERWRITTEN an existing handle.

use super::CreateShapeModel;
use crate::delete_shape_model;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_shape_model::CreateShapeModel> + From<crate::delete_shape_model::DeleteShapeModel>>(_payload: &CreateShapeModel, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match &base.shape_model {
        Some(existing) => vec![Op::from(CreateShapeModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => vec![Op::from(delete_shape_model::DeleteShapeModel {})],
    })
}
//#endregion 🔖️Inverse
