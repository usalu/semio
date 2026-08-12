//! 🔺️ `rotate-object` — sparse diff construction: whole-transform patch with only `rotation` changed.

use super::mutation::RotateObject;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot, LowpolyTransform};

//#region 🔖️Diff
pub fn diff(payload: &RotateObject, base: &LowpolySnapshot) -> LowpolyDiff {
    let transform = base
        .objects
        .iter()
        .find(|object| object.id == payload.id)
        .map(|object| LowpolyTransform { rotation: payload.new_rotation, ..object.transform.clone() })
        .unwrap_or(LowpolyTransform { rotation: payload.new_rotation, ..LowpolyTransform::default() });
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { transform: Some(transform), ..LowpolyObjectPatch::default() })
}
//#endregion 🔖️Diff
