//! ↩️ `scale-selection` — undo writes the base position of every vertex the motion moves back as ONE `set-vertex-positions`
//! (`crate::mutations::lowpoly_selection_motion_inverse`, read off `base` and the payload); a refused motion or one that moves
//! nothing inverts to nothing.

use super::ScaleSelection;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ScaleSelection, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    Ok({
    if payload.invariant_violation().is_some() {
        return Ok(Vec::new());
    }
    crate::mutations::lowpoly_selection_motion_inverse(base, &payload.object_id, &payload.vertex_ids, &payload.motion())?

    })
}
//#endregion 🔖️Inverse
