//! ↩️ `create-shape-model` — undo restores whichever handle occupied `shape_model` BEFORE this create ran
//! (a real prior handle if the slot was occupied, or `delete-shape-model` if it was empty) — never a
//! bare "delete", since `create-shape-model` may have OVERWRITTEN an existing handle.

use super::mutation::CreateShapeModel;
use crate::artifacts::cad::mutations::{delete_shape_model, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &CreateShapeModel, base: &CadSnapshot) -> Vec<CadMutation> {
    match &base.shape_model {
        Some(existing) => vec![CadMutation::CreateShapeModel(CreateShapeModel { child_id: existing.child_id.clone(), target: existing.target.to_uri() })],
        None => vec![CadMutation::DeleteShapeModel(delete_shape_model::mutation::DeleteShapeModel {})],
    }
}
//#endregion 🔖️Inverse
