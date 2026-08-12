//! 🔺️ `create-object` — sparse diff construction (delegates to the existing objects-add field-delta
//! constructor, which already builds a sparse `LowpolyDiff` and is a no-op for a duplicate id).

use super::mutation::CreateObject;
use crate::artifacts::lowpoly::diff::diff_objects_add;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateObject, base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_add(payload.index, payload.object.clone(), base)
}
//#endregion 🔖️Diff
