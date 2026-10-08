//! 🔺️ `rotate-selection` — sparse diff construction: Fatal `invariant` for the payload's own breach (a vertex named twice, a non-finite pivot, axis or angle, or the zero axis (`x-semio-invariant` `axis-nonzero`)),
//! then the shared selection motion (`crate::mutations::lowpoly_selection_motion_diff`) reads the BASE vertex positions and states
//! one absolute position row per vertex it moves: Error `target-missing`, Warning `partial` and Warning `no-op` come from there.

use super::RotateSelection;
use crate::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RotateSelection, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if let Some(reason) = payload.invariant_violation() {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, [payload.object_id.clone()]);
    }
    crate::mutations::lowpoly_selection_motion_diff(base, &payload.object_id, &payload.vertex_ids, &payload.motion())
}
//#endregion 🔖️Diff
