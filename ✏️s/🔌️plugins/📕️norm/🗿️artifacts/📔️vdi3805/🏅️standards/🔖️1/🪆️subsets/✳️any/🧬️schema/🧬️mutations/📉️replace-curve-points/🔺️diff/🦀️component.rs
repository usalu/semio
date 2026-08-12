//! 🔺️ `replace-curve-points` — sparse diff construction; missing id is a no-op clone.

use super::mutation::ReplaceCurvePoints;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceCurvePoints, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut curves = base.curves.clone();
    if let Some(curve) = curves.get_mut(&payload.id) {
        curve.points = payload.new_points.clone();
    }
    Vdi3805Diff { curves: Some(curves), ..Default::default() }
}
//#endregion 🔖️Diff
