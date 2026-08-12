//! 🔺️ `delete-drawing` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::DeleteDrawing;
use crate::artifacts::cad::diff::{CadDiff, CadDrawingChildList};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteDrawing, base: &CadSnapshot) -> CadDiff {
    let drawings: Vec<_> = base.drawings.iter().filter(|c| c.child_id != payload.child_id).cloned().collect();
    CadDiff { drawings: Some(CadDrawingChildList { values: drawings }), ..Default::default() }
}
//#endregion 🔖️Diff
