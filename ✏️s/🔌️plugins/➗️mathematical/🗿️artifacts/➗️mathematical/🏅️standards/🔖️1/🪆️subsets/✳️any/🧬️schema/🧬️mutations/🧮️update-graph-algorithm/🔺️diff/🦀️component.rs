//! 🔺️ `update-graph-algorithm` — sparse diff construction.

use super::mutation::UpdateGraphAlgorithm;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateGraphAlgorithm, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = mathematical_graph(base);
    graph.algorithm = payload.new_algorithm.clone();
    graph.algorithm_seed = payload.new_algorithm_seed.clone();
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
