//! ↩️ `create-drawing` — undo is `delete-drawing` for the just-minted `child_id`.

use super::CreateDrawing;
use crate::delete_drawing;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::delete_drawing::DeleteDrawing>>(payload: &CreateDrawing, _base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(vec![Op::from(delete_drawing::DeleteDrawing { child_id: payload.child_id.clone() })])
}
//#endregion 🔖️Inverse
