//! 🔺️ `move-point` — sparse diff construction.

use super::mutation::MovePoint;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MovePoint, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut geometry = mathematical_geometry(base);
    if let Some(point) = geometry.points.get_mut(payload.index) {
        point.x = payload.x;
        point.y = payload.y;
    }
    let (notation, results, computed) = mathematical_children_from_state(&mathematical_graph(base), &geometry);
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
