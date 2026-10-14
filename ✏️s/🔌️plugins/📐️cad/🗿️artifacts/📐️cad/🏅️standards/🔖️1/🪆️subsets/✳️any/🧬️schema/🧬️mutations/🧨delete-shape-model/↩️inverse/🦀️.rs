//! ↩️ `delete-shape-model` — undo is `create-shape-model` with the escrowed handle captured from BASE;
//! empty (`Vec::new()`) when the slot was already absent (nothing to undo).

use super::DeleteShapeModel;
use crate::create_shape_model;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_shape_model::CreateShapeModel>>(_payload: &DeleteShapeModel, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match &base.shape_model {
        Some(existing) => vec![Op::from(create_shape_model::CreateShapeModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
