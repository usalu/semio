//! 🔺️ `change-curve-points` — sparse diff construction; missing id is
//! `mutation.target-missing`.

use super::ChangeCurvePoints;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805CurvesRows, Vdi3805CurvesPatch};

//#region 🔖️Diff

pub fn diff(payload: &ChangeCurvePoints, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(curve) = base.curves.get(&payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Curve \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if curve.points == payload.new_points {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Curve \"{}\" already has these points.", payload.id));
    }
    protocol::MutationOutcome::new(Vdi3805Diff {
        curves: Some(Vdi3805CurvesRows { modified: vec![Vdi3805CurvesPatch { key: payload.id.clone(), points: Some(payload.new_points.clone()) }], ..Default::default() }),
        ..Default::default()
    })
}
