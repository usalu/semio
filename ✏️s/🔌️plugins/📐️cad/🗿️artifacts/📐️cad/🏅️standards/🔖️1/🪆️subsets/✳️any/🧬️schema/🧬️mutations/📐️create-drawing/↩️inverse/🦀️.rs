//! ↩️ `create-drawing` — undo is `delete-drawing` for the just-minted `child_id`.

use super::CreateDrawing;
use crate::mutations::{delete_drawing, CadMutation};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateDrawing, _base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![CadMutation::DeleteDrawing(delete_drawing::DeleteDrawing { child_id: payload.child_id.clone() })]

    })())
}
//#endregion 🔖️Inverse
