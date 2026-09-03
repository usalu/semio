//! 🔺️ `remove-point` — sparse diff construction.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ Out-of-range `index` is Error `target-missing`.
pub async fn diff(payload: &super::RemovePoint, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut geometry = equation_geometry(base);
    if payload.index >= geometry.points.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Point at index {} does not exist.", payload.index), [payload.index.to_string()]);
    }
    geometry.points.remove(payload.index);
    let (notation, results, computed) = equation_children_from_state(&equation_graph(base), &geometry);
    protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
