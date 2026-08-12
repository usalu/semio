//! 🔺️ Sparse diff builder for `DragObjects` — offsets every selected object's own current origin.
use super::mutation::DragObjects;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::mutations::{transform_objects_diff, CadObjectPatch};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DragObjects, base: &CadSnapshot) -> CadDiff {
    transform_objects_diff(base, &payload.object_ids, |object| CadObjectPatch { origin: Some([object.origin[0] + payload.dx, object.origin[1] + payload.dy, object.origin[2] + payload.dz]), ..Default::default() })
}
//#endregion 🔖️Diff
