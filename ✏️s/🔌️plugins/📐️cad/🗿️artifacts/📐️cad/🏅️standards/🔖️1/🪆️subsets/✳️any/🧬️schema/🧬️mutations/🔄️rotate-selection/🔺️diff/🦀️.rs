//! 🔺️ Sparse diff builder for `RotateSelection` — every addressed object's BASE orientation is composed with the payload
//! turn and the pane's composed model child is re-materialized; a zero-length axis is the Fatal `axis-nonzero`.
use super::RotateSelection;
use crate::diff::CadDiff;
use crate::mutations::rotate_objects::inverse::CAD_IDENTITY_ORIENTATION;
use crate::mutations::{cad_quaternion_product, cad_selection_diff};
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RotateSelection, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(delta) = payload.delta() else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "axis-nonzero: a turn needs a finite angle about a finite axis of non-zero length", payload.targets.clone());
    };
    cad_selection_diff(payload.pane, &payload.targets, payload.angle == 0.0, base, |object| {
        let next = cad_quaternion_product(delta, object.orientation.unwrap_or(CAD_IDENTITY_ORIENTATION));
        object.orientation = (next != CAD_IDENTITY_ORIENTATION).then_some(next);
    })
}
//#endregion 🔖️Diff
