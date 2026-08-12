//! 🔺️ `change-object-smooth-shading` — sparse diff construction: one-field object patch.

use super::mutation::ChangeObjectSmoothShading;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeObjectSmoothShading, _base: &LowpolySnapshot) -> LowpolyDiff {
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { smooth_shading: Some(payload.new_smooth_shading), ..LowpolyObjectPatch::default() })
}
//#endregion 🔖️Diff
