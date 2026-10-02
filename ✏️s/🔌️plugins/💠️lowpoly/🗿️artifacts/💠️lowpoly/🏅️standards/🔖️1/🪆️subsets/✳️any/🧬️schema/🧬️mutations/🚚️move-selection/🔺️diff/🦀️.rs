//! 🔺️ `move-selection` — sparse diff construction: Fatal `invariant` for the payload's own breach (a vertex named twice or a non-finite offset),
//! then the shared selection motion (`crate::mutations::lowpoly_selection_motion_diff`) re-encodes the object's mesh
//! content and the handle it hashes to: Error `target-missing`, Warning `partial` and Warning `no-op` come from there.

use super::MoveSelection;
use crate::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MoveSelection, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if let Some(reason) = payload.invariant_violation() {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, [payload.object_id.clone()]);
    }
    crate::mutations::lowpoly_selection_motion_diff(base, &payload.object_id, &payload.vertex_ids, &payload.motion())
}
//#endregion 🔖️Diff
