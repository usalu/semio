//! 🔺️ `insert-point` — sparse diff construction.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationPoint, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ Ascending-insert-clamped: an out-of-range `index` lands at the end rather than panicking,
/// reported as Warning `clamped`.
pub async fn diff(payload: &super::InsertPoint, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut geometry = equation_geometry(base);
    let index = payload.index.min(geometry.points.len());
    let was_clamped = index != payload.index;
    geometry.points.insert(index, EquationPoint { x: payload.x, y: payload.y });
    let (notation, results, computed) = equation_children_from_state(&equation_graph(base), &geometry);
    let outcome = protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() });
    if was_clamped {
        outcome.warn("mutation.clamped", format!("Insert index {} was out of range and clamped to {}.", payload.index, index))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
