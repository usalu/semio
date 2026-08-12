//! 🔺️ Sparse diff builder for `RotateObjects` — composes the axis-angle delta onto each object's
//! current orientation.
use super::mutation::RotateObjects;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::mutations::{quat_from_axis_angle, quat_mul, transform_objects_diff, CadObjectPatch};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RotateObjects, base: &CadSnapshot) -> CadDiff {
    let delta = quat_from_axis_angle(payload.ax, payload.ay, payload.az, payload.angle);
    transform_objects_diff(base, &payload.object_ids, |object| {
        let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        CadObjectPatch { orientation: Some(quat_mul(delta, current)), ..Default::default() }
    })
}
//#endregion 🔖️Diff
