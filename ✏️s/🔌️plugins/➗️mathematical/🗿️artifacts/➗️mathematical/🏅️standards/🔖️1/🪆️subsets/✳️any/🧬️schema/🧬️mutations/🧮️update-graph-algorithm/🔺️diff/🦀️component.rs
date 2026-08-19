//! 🔺️ `update-graph-algorithm` — sparse diff construction.

use super::mutation::UpdateGraphAlgorithm;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &UpdateGraphAlgorithm, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    let mut graph = mathematical_graph(base);
    if graph.algorithm == payload.new_algorithm && graph.algorithm_seed == payload.new_algorithm_seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Graph algorithm is already \"{}\".", payload.new_algorithm));
    }
    graph.algorithm = payload.new_algorithm.clone();
    graph.algorithm_seed = payload.new_algorithm_seed.clone();
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
