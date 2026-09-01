//! 🔺️ `delete-drawing` — sparse diff construction, built directly from `(payload, base)`.

use super::DeleteDrawing;
use crate::artifacts::cad::diff::{CadDiff, CadDrawingChildList};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteDrawing, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if !base.drawings.iter().any(|c| c.child_id == payload.child_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Drawing \"{}\" does not exist.", payload.child_id), [payload.child_id.clone()]);
    }
    let drawings: Vec<_> = base.drawings.iter().filter(|c| c.child_id != payload.child_id).cloned().collect();
    protocol::MutationOutcome::new(CadDiff { drawings: Some(CadDrawingChildList { values: drawings }), ..Default::default() })
}
//#endregion 🔖️Diff
