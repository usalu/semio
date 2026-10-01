//! 🔺️ `move-selection` — sparse diff construction through the shared selection motion
//! (`crate::mutations::lowpoly_selection_motion_diff`): the object's re-encoded mesh content and the handle it hashes to.

use super::MoveSelection;
use crate::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MoveSelection, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    crate::mutations::lowpoly_selection_motion_diff(base, &payload.object_id, &payload.vertex_ids, &payload.motion())
}
//#endregion 🔖️Diff
