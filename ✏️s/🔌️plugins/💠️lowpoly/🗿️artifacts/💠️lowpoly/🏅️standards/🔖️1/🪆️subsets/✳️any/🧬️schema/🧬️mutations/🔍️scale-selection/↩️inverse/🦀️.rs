//! ↩️ `scale-selection` — undo restores the object's prior mesh handle and content as ONE `create-mesh`
//! (`crate::mutations::lowpoly_selection_motion_inverse` over this leaf's own diff); a refused motion or one that moves
//! nothing inverts to nothing.

use super::ScaleSelection;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ScaleSelection, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    Ok({
    crate::mutations::lowpoly_selection_motion_inverse(base, &payload.object_id, &super::diff::diff(payload, base))?

    })
}
//#endregion 🔖️Inverse
