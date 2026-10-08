//! 🔺️ `remove-point` — sparse diff construction: one positional remove.
use crate::diff::{EquationPointEdit, EquationPointsDelta};
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ Out-of-range `index` is Error `target-missing`.
pub fn diff(payload: &super::RemovePoint, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if payload.index >= base.geometry.points.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Point at index {} does not exist.", payload.index), [payload.index.to_string()]);
    }
    let diff = EquationDiff { points: Some(EquationPointsDelta { edits: vec![EquationPointEdit::Remove { at: payload.index as u32 }] }), ..Default::default() };
    protocol::MutationOutcome::new(crate::equation_state_diff(diff, base))
}
//#endregion 🔖️Diff
