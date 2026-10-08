//! 🔺️ `insert-point` — sparse diff construction: one positional insert.
use crate::diff::{EquationPointEdit, EquationPointsDelta};
use crate::{EquationDiff, EquationPoint, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ Ascending-insert-clamped: an out-of-range `index` lands at the end rather than panicking,
/// reported as Warning `clamped`.
pub fn diff(payload: &super::InsertPoint, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let index = payload.index.min(base.geometry.points.len());
    let was_clamped = index != payload.index;
    let edit = EquationPointEdit::Insert { at: index as u32, point: EquationPoint { x: payload.x, y: payload.y } };
    let diff = EquationDiff { points: Some(EquationPointsDelta { edits: vec![edit] }), ..Default::default() };
    let outcome = protocol::MutationOutcome::new(crate::equation_state_diff(diff, base));
    if was_clamped {
        outcome.warning("mutation.clamped", format!("Insert index {} was out of range and clamped to {}.", payload.index, index))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
