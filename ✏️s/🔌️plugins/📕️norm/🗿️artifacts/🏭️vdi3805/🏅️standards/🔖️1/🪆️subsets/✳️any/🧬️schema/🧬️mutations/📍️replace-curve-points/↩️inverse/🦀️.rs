//! ↩️ `replace-curve-points` — undo restores BASE's points; missing id ⇒ `Vec::new()`.

use super::ReplaceCurvePoints;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceCurvePoints, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let Some(curve) = base.curves.get(&payload.id) else {
        return Vec::new();
    };
    vec![Vdi3805Mutation::ReplaceCurvePoints(ReplaceCurvePoints { id: payload.id.clone(), new_points: curve.points.clone() })]
}
//#endregion 🔖️Inverse
