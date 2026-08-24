//! ↩️ `delete-drawing` — undo is `create-drawing` with the escrowed handle from BASE; empty when
//! absent.

use super::mutation::DeleteDrawing;
use crate::artifacts::cad::mutations::{create_drawing, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteDrawing, base: &CadSnapshot) -> Vec<CadMutation> {
    match base.drawings.iter().find(|c| c.child_id == payload.child_id) {
        Some(existing) => vec![CadMutation::CreateDrawing(create_drawing::mutation::CreateDrawing { child_id: existing.child_id.clone(), target: existing.target.to_uri() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
