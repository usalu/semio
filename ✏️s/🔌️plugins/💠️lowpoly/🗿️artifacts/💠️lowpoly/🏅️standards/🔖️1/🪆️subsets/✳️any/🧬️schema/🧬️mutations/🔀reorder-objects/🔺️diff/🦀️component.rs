//! 🔺️ `reorder-objects` — sparse diff construction (delegates to the existing objects-move field-delta
//! constructor).

use super::mutation::ReorderObjects;
use crate::artifacts::lowpoly::diff::diff_objects_move;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReorderObjects, base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_move(&payload.id, payload.to_index, base)
}
//#endregion 🔖️Diff
