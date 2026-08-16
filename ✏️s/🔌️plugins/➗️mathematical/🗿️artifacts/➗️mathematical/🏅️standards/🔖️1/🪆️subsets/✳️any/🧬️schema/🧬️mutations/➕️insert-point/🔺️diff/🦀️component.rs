//! 🔺️ `insert-point` — sparse diff construction.

use super::mutation::InsertPoint;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalPoint, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ Ascending-insert-clamped: an out-of-range `index` lands at the end rather than panicking,
/// reported as Warning `clamped`.
pub fn diff(payload: &InsertPoint, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    let mut geometry = mathematical_geometry(base);
    let index = payload.index.min(geometry.points.len());
    let was_clamped = index != payload.index;
    geometry.points.insert(index, MathematicalPoint { x: payload.x, y: payload.y });
    let (notation, results, computed) = mathematical_children_from_state(&mathematical_graph(base), &geometry);
    let outcome = protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() });
    if was_clamped {
        outcome.warn("mutation.clamped", format!("Insert index {} was out of range and clamped to {}.", payload.index, index))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
