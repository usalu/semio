//! 🔺️ Sparse diff builder for `DragSelection` — every addressed object of the pane moves by the payload offset, read
//! off the BASE origin, and the pane's composed model child is re-materialized.
use super::DragSelection;
use crate::diff::CadDiff;
use crate::mutations::cad_selection_diff;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DragSelection, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if payload.offset.iter().any(|component| !component.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    let offset = payload.offset;
    cad_selection_diff(payload.pane, &payload.targets, offset == [0.0; 3], base, |object| object.origin = [object.origin[0] + offset[0], object.origin[1] + offset[1], object.origin[2] + offset[2]])
}
//#endregion 🔖️Diff
