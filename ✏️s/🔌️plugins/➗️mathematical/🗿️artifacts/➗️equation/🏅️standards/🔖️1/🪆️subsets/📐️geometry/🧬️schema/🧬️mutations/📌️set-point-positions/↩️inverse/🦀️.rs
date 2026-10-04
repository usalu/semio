//! ↩️ `set-point-positions` — ONE `set-point-positions` row with every placed point's BASE position; a placement that moves
//! nothing has no inverse.

use super::{equation_point_targets_invariant, EquationPointPosition, SetPointPositions};
use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &SetPointPositions, base: &EquationSnapshot) -> Vec<EquationMutation> {
    if equation_point_targets_invariant(&payload.indices()).is_err() || payload.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
        return Vec::new();
    }
    let placed: Vec<(EquationPointPosition, bool)> = payload
        .positions
        .iter()
        .filter_map(|position| base.geometry.points.get(position.index).map(|point| (EquationPointPosition { index: position.index, x: point.x, y: point.y }, (point.x, point.y) != (position.x, position.y))))
        .collect();
    if !placed.iter().any(|(_, moves)| *moves) {
        return Vec::new();
    }
    vec![EquationMutation::SetPointPositions(SetPointPositions { positions: placed.into_iter().map(|(position, _)| position).collect() })]
}
//#endregion 🔖️Inverse
