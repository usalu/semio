//! 🔺️ `update-graph-algorithm` — sparse diff construction.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::UpdateGraphAlgorithm, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = equation_graph(base);
    if graph.algorithm == payload.new_algorithm && graph.algorithm_seed == payload.new_algorithm_seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Graph algorithm is already \"{}\".", payload.new_algorithm));
    }
    graph.algorithm = payload.new_algorithm.clone();
    graph.algorithm_seed = payload.new_algorithm_seed.clone();
    let (notation, results, computed) = equation_children_from_state(&graph, &equation_geometry(base));
    protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
