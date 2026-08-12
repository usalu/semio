//! 🔺️ `replace-graph` — sparse diff construction.

use super::mutation::ReplaceGraph;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceGraph, _base: &MathematicalSnapshot) -> MathematicalDiff {
    MathematicalDiff { graph: Some(payload.graph.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
