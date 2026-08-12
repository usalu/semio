//! 🔺️ `delete-object` — sparse diff construction (delegates to the existing objects-remove field-delta
//! constructor).

use super::mutation::DeleteObject;
use crate::artifacts::lowpoly::diff::diff_objects_remove;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteObject, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_remove(payload.id.clone())
}
//#endregion 🔖️Diff
