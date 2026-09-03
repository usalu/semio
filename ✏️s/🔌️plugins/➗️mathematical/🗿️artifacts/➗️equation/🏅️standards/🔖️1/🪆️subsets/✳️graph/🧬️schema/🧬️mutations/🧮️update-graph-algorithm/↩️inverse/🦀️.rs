//! ↩️ `update-graph-algorithm` — undo reconstructed from BASE state.

use crate::artifacts::equation::{equation_graph, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::UpdateGraphAlgorithm, base: &EquationSnapshot) -> Vec<EquationMutation> {
    let graph = crate::artifacts::equation::equation_graph(base);
    vec![EquationMutation::UpdateGraphAlgorithm(super::UpdateGraphAlgorithm { new_algorithm: graph.algorithm, new_algorithm_seed: graph.algorithm_seed })]
}
//#endregion 🔖️Inverse
