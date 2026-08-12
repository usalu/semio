//! 🔺️ `update-graph-algorithm` — sparse diff construction.

use super::mutation::UpdateGraphAlgorithm;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateGraphAlgorithm, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = base.graph.clone();
    graph.algorithm = payload.new_algorithm.clone();
    graph.algorithm_seed = payload.new_algorithm_seed.clone();
    MathematicalDiff { graph: Some(graph), ..Default::default() }
}
//#endregion 🔖️Diff
