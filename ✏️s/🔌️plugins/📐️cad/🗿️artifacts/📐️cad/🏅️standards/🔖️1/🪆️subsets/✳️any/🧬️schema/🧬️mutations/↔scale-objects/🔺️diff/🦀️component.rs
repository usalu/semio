//! 🔺️ Sparse diff builder for `ScaleObjects` — composes the per-axis factor onto each object's
//! current scale.
use super::mutation::ScaleObjects;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::mutations::{transform_objects_diff, CadObjectPatch};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ScaleObjects, base: &CadSnapshot) -> CadDiff {
    transform_objects_diff(base, &payload.object_ids, |object| {
        let current = object.scale.unwrap_or([1.0, 1.0, 1.0]);
        CadObjectPatch { scale: Some([current[0] * payload.sx, current[1] * payload.sy, current[2] * payload.sz]), ..Default::default() }
    })
}
//#endregion 🔖️Diff
