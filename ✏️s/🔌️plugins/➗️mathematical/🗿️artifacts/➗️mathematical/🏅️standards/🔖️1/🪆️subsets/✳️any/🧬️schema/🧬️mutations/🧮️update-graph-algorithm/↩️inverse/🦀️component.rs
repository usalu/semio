//! ↩️ `update-graph-algorithm` — undo reconstructed from BASE state.

use super::mutation::UpdateGraphAlgorithm;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateGraphAlgorithm, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let graph = crate::artifacts::mathematical::mathematical_graph(base);
    vec![MathematicalMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: graph.algorithm, new_algorithm_seed: graph.algorithm_seed })]
}
//#endregion 🔖️Inverse
