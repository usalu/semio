//! 🔺️ `change-graph-directed` — sparse diff construction.

use super::mutation::ChangeGraphDirected;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ Clones the current graph and flips only the `directed` field, wrapping the whole (still
/// sparse at the snapshot level) graph slot — `MathematicalDiff` has no finer-than-`graph`
/// granularity, so every graph-scoped mutation shares this "clone + patch one field" shape.
pub fn diff(payload: &ChangeGraphDirected, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = base.graph.clone();
    graph.directed = payload.new_directed;
    MathematicalDiff { graph: Some(graph), ..Default::default() }
}
//#endregion 🔖️Diff
