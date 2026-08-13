//! 🔺️ `replace-points` — sparse diff construction.

use super::mutation::ReplacePoints;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_graph, MathematicalDiff, MathematicalGeometry, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplacePoints, base: &MathematicalSnapshot) -> MathematicalDiff {
    let geometry = MathematicalGeometry { points: payload.points.clone() };
    let (notation, results, computed) = mathematical_children_from_state(&mathematical_graph(base), &geometry);
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
