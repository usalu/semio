//! 🔺️ `insert-point` — sparse diff construction.

use crate::{EquationDiff, EquationPoint, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ Ascending-insert-clamped: an out-of-range `index` lands at the end rather than panicking,
/// reported as Warning `clamped`.
pub fn diff(payload: &super::InsertPoint, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut geometry = base.geometry.clone();
    let index = payload.index.min(geometry.points.len());
    let was_clamped = index != payload.index;
    geometry.points.insert(index, EquationPoint { x: payload.x, y: payload.y });
    let outcome = protocol::MutationOutcome::new(crate::equation_state_diff(base.graph.clone(), geometry));
    if was_clamped {
        outcome.warning("mutation.clamped", format!("Insert index {} was out of range and clamped to {}.", payload.index, index))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
