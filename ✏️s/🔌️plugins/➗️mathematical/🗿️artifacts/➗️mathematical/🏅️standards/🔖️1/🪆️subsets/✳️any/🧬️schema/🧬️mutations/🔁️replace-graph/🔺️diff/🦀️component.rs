//! 🔺️ `replace-graph` — sparse diff construction.

use super::mutation::ReplaceGraph;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceGraph, base: &MathematicalSnapshot) -> MathematicalDiff {
    let (notation, results, computed) = mathematical_children_from_state(&payload.graph, &mathematical_geometry(base));
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
