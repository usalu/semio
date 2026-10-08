//! ↩️ `delete-drawing` — undo is `create-drawing` with the escrowed handle from BASE; empty when
//! absent.

use super::DeleteDrawing;
use crate::mutations::{create_drawing, CadMutation};
use crate::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteDrawing, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
    Ok(match base.drawings.iter().enumerate().find(|(_, c)| c.child_id == payload.child_id) {
        Some((index, existing)) => vec![CadMutation::CreateDrawing(create_drawing::CreateDrawing { child_id: existing.child_id.clone(), target: existing.target.clone(), index: u32::try_from(index).ok() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
