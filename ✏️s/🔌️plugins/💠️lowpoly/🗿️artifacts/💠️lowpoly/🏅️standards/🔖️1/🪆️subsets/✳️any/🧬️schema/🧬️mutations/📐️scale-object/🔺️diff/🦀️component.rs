//! 🔺️ `scale-object` — sparse diff construction: whole-transform patch with only `scale` changed.

use super::mutation::ScaleObject;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot, LowpolyTransform};

//#region 🔖️Diff
pub fn diff(payload: &ScaleObject, base: &LowpolySnapshot) -> LowpolyDiff {
    let transform = base
        .objects
        .iter()
        .find(|object| object.id == payload.id)
        .map(|object| LowpolyTransform { scale: payload.new_scale, ..object.transform.clone() })
        .unwrap_or(LowpolyTransform { scale: payload.new_scale, ..LowpolyTransform::default() });
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { transform: Some(transform), ..LowpolyObjectPatch::default() })
}
//#endregion 🔖️Diff
