//! 🔺️ `create-drawing` — sparse diff construction from an exact composed drawing child handle.

use super::CreateDrawing;
use crate::diff::{CadDiff, CadDrawingsDelta};
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateDrawing, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let candidate = match crate::cad_drawing_child(&payload.child_id, &payload.target) {
        Ok(candidate) => candidate,
        Err(reason) => return protocol::MutationOutcome::fatal("mutation.invariant", reason, [payload.child_id.clone()]),
    };
    if base.drawings.iter().any(|drawing| drawing.child_id == payload.child_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A drawing with id \"{}\" already exists.", payload.child_id), [payload.child_id.clone()]);
    }
    let index = payload.index.map_or(base.drawings.len(), |index| index as usize);
    if index > base.drawings.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Drawing insertion index {index} exceeds the {} existing drawings.", base.drawings.len()), [payload.child_id.clone()]);
    }
    protocol::MutationOutcome::new(CadDiff { drawings: Some(CadDrawingsDelta::insertion(index, candidate)), ..Default::default() })
}
//#endregion 🔖️Diff
