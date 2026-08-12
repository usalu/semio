//! 🔺️ `rename-object` — sparse diff construction: one-field object patch on `name`.

use super::mutation::RenameObject;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameObject, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { name: Some(payload.new_name.clone()), ..LowpolyObjectPatch::default() })
}
//#endregion 🔖️Diff
