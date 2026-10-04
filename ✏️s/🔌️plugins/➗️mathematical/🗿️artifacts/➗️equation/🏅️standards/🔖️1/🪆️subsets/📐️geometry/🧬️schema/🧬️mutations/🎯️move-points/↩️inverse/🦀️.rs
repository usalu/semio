//! ↩️ `move-points` — ONE absolute `set-point-positions` row putting every moved point back at its BASE position (never a
//! negated offset), so a multi-point drag stays one point-invertible row.

use crate::standards::v1::subsets::geometry::schema::mutations::set_point_positions::{equation_point_targets_invariant, EquationPointPosition, SetPointPositions};
use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::MovePoints, base: &EquationSnapshot) -> Vec<EquationMutation> {
    if equation_point_targets_invariant(&payload.indices).is_err() || !payload.dx.is_finite() || !payload.dy.is_finite() || (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    let positions: Vec<EquationPointPosition> = payload.indices.iter().filter_map(|index| base.geometry.points.get(*index).map(|point| EquationPointPosition { index: *index, x: point.x, y: point.y })).collect();
    if positions.is_empty() {
        return Vec::new();
    }
    vec![EquationMutation::SetPointPositions(SetPointPositions { positions })]
}
//#endregion 🔖️Inverse
