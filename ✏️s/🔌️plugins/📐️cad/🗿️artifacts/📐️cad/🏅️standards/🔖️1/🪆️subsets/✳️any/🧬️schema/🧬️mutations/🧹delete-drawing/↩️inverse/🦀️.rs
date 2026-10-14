//! ↩️ `delete-drawing` — undo is `create-drawing` with the escrowed handle from BASE; empty when
//! absent.

use super::DeleteDrawing;
use crate::create_drawing;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_drawing::CreateDrawing>>(payload: &DeleteDrawing, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match base.drawings.iter().enumerate().find(|(_, c)| c.child_id == payload.child_id) {
        Some((index, existing)) => vec![Op::from(create_drawing::CreateDrawing { child_id: existing.child_id.clone(), target: existing.target.clone(), index: u32::try_from(index).ok() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
