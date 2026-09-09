//! 🔺️ `create-drawing` — sparse diff construction from an exact composed drawing child handle.

use super::CreateDrawing;
use crate::diff::{CadDiff, CadDrawingChildList};
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateDrawing, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let candidate = match crate::cad_drawing_child_from_uri(&payload.child_id, &payload.target) {
        Ok(candidate) => candidate,
        Err(reason) => return protocol::MutationOutcome::fatal("mutation.child-identity", reason, [payload.child_id.clone()]),
    };
    if base.drawings.iter().any(|drawing| drawing.child_id == payload.child_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A drawing with id \"{}\" already exists.", payload.child_id), [payload.child_id.clone()]);
    }
    let mut drawings = base.drawings.clone();
    drawings.push(candidate);
    protocol::MutationOutcome::new(CadDiff { drawings: Some(CadDrawingChildList { values: drawings }), ..Default::default() })
}
//#endregion 🔖️Diff
