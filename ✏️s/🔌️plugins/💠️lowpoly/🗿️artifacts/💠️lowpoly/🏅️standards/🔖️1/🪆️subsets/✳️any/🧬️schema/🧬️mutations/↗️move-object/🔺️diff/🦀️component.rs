//! 🔺️ `move-object` — sparse diff construction: whole-transform patch with only `position` changed
//! (storage only supports a whole `LowpolyTransform` slot, so the untouched fields are read from base).

use super::mutation::MoveObject;
use crate::artifacts::lowpoly::diff::diff_objects_patch;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyObjectPatch, LowpolySnapshot, LowpolyTransform};

//#region 🔖️Diff
pub fn diff(payload: &MoveObject, base: &LowpolySnapshot) -> LowpolyDiff {
    let transform = base
        .objects
        .iter()
        .find(|object| object.id == payload.id)
        .map(|object| LowpolyTransform { position: payload.new_position, ..object.transform.clone() })
        .unwrap_or(LowpolyTransform { position: payload.new_position, ..LowpolyTransform::default() });
    diff_objects_patch(payload.id.clone(), LowpolyObjectPatch { transform: Some(transform), ..LowpolyObjectPatch::default() })
}
//#endregion 🔖️Diff
