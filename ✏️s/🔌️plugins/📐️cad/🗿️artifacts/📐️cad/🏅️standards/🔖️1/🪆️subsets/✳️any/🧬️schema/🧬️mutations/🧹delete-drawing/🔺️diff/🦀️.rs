//! 🔺️ `delete-drawing` — sparse diff construction, built directly from `(payload, base)`.

use super::DeleteDrawing;
use crate::diff::{CadDiff, CadDrawingsDelta};
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteDrawing, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(index) = base.drawings.iter().position(|c| c.child_id == payload.child_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Drawing \"{}\" does not exist.", payload.child_id), [payload.child_id.clone()]);
    };
    protocol::MutationOutcome::new(CadDiff { drawings: Some(CadDrawingsDelta::removal(&base.drawings, index)), ..Default::default() })
}
//#endregion 🔖️Diff
