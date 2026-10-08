//! 🔺️ `scale-selection` — sparse diff construction: Fatal `invariant` for the payload's own breach (a vertex named twice, a non-finite pivot or a factor that is not positive and finite),
//! then the shared selection motion (`crate::mutations::lowpoly_selection_motion_diff`) reads the BASE vertex positions and states
//! one absolute position row per vertex it moves: Error `target-missing`, Warning `partial` and Warning `no-op` come from there.

use super::ScaleSelection;
use crate::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ScaleSelection, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if let Some(reason) = payload.invariant_violation() {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, [payload.object_id.clone()]);
    }
    crate::mutations::lowpoly_selection_motion_diff(base, &payload.object_id, &payload.vertex_ids, &payload.motion())
}
//#endregion 🔖️Diff
