//! ↩️ `create-drawing` — undo is `delete-drawing` for the just-minted `child_id`.

use super::mutation::CreateDrawing;
use crate::artifacts::cad::mutations::{delete_drawing, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateDrawing, _base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::DeleteDrawing(delete_drawing::mutation::DeleteDrawing { child_id: payload.child_id.clone() })]
}
//#endregion 🔖️Inverse
