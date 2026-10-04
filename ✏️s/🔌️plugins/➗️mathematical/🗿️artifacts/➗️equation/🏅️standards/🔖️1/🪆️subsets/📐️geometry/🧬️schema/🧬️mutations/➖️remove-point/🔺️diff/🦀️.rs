//! 🔺️ `remove-point` — sparse diff construction.

use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ Out-of-range `index` is Error `target-missing`.
pub fn diff(payload: &super::RemovePoint, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut geometry = base.geometry.clone();
    if payload.index >= geometry.points.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Point at index {} does not exist.", payload.index), [payload.index.to_string()]);
    }
    geometry.points.remove(payload.index);
    protocol::MutationOutcome::new(crate::equation_state_diff(base.graph.clone(), geometry))
}
//#endregion 🔖️Diff
