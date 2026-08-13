//! 🔺️ `remove-point` — sparse diff construction.

use super::mutation::RemovePoint;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ Out-of-range `index` is a no-op — the clone is returned unchanged.
pub fn diff(payload: &RemovePoint, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut geometry = mathematical_geometry(base);
    if payload.index < geometry.points.len() {
        geometry.points.remove(payload.index);
    }
    let (notation, results, computed) = mathematical_children_from_state(&mathematical_graph(base), &geometry);
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
