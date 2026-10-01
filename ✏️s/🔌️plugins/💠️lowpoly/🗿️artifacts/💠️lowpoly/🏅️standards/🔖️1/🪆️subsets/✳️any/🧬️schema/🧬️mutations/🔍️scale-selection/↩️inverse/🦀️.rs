//! ↩️ `scale-selection` — undo restores the object's prior mesh handle and content as ONE `create-mesh`
//! (`crate::mutations::lowpoly_selection_motion_inverse`); a motion that moves nothing inverts to nothing.

use super::ScaleSelection;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ScaleSelection, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    crate::mutations::lowpoly_selection_motion_inverse(base, &payload.object_id, &payload.vertex_ids, &payload.motion())
}
//#endregion 🔖️Inverse
