//! ↩️ `delete-shape-model` — undo is `create-shape-model` with the escrowed handle captured from BASE;
//! empty (`Vec::new()`) when the slot was already absent (nothing to undo).

use super::DeleteShapeModel;
use crate::artifacts::cad::mutations::{create_shape_model, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &DeleteShapeModel, base: &CadSnapshot) -> Vec<CadMutation> {
    match &base.shape_model {
        Some(existing) => vec![CadMutation::CreateShapeModel(create_shape_model::CreateShapeModel { child_id: existing.child_id.clone(), target: existing.target.to_uri() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
