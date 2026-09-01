//! 🔺️ `replace-curve-points` — sparse diff construction; missing id is
//! `mutation.target-missing`.

use super::ReplaceCurvePoints;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceCurvePoints, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(curve) = base.curves.get(&payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Curve \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if curve.points == payload.new_points {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Curve \"{}\" already has these points.", payload.id));
    }
    let mut curves = base.curves.clone();
    if let Some(curve) = curves.get_mut(&payload.id) {
        curve.points = payload.new_points.clone();
    }
    protocol::MutationOutcome::new(Vdi3805Diff { curves: Some(curves), ..Default::default() })
}
//#endregion 🔖️Diff
