//! 🔺️ `update-graph-algorithm` — sparse diff construction.

use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateGraphAlgorithm, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = base.graph.clone();
    if graph.algorithm == payload.new_algorithm && graph.algorithm_seed == payload.new_algorithm_seed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Graph algorithm is already \"{}\".", payload.new_algorithm));
    }
    graph.algorithm = payload.new_algorithm.clone();
    graph.algorithm_seed = payload.new_algorithm_seed.clone();
    protocol::MutationOutcome::new(crate::equation_state_diff(graph, base.geometry.clone()))
}
//#endregion 🔖️Diff
