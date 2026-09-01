//! ↩️ `create-drawing` — undo is `delete-drawing` for the just-minted `child_id`.

use super::CreateDrawing;
use crate::artifacts::cad::mutations::{delete_drawing, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateDrawing, _base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::DeleteDrawing(delete_drawing::DeleteDrawing { child_id: payload.child_id.clone() })]
}
//#endregion 🔖️Inverse
