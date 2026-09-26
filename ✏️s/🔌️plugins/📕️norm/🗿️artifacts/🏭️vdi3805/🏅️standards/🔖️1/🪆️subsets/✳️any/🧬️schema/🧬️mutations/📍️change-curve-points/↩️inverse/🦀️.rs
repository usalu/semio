//! ↩️ `change-curve-points` — undo restores BASE's points; missing id ⇒ `Vec::new()`.

use super::ChangeCurvePoints;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeCurvePoints, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let Some(curve) = base.curves.get(&payload.id) else {
        return Vec::new();
    };
    vec![Vdi3805Mutation::ChangeCurvePoints(ChangeCurvePoints { id: payload.id.clone(), new_points: curve.points.clone() })]
}
//#endregion 🔖️Inverse
