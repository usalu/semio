//! ↩️ `update-graph-algorithm` — undo reconstructed from BASE state.

use crate::artifacts::mathematical::{mathematical_graph, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::UpdateGraphAlgorithm, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let graph = crate::artifacts::mathematical::mathematical_graph(base);
    vec![MathematicalMutation::UpdateGraphAlgorithm(super::UpdateGraphAlgorithm { new_algorithm: graph.algorithm, new_algorithm_seed: graph.algorithm_seed })]
}
//#endregion 🔖️Inverse
