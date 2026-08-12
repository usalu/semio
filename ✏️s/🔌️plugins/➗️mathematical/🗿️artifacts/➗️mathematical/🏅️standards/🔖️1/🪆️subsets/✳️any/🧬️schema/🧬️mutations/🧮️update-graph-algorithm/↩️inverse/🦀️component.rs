//! ↩️ `update-graph-algorithm` — undo reconstructed from BASE state.

use super::mutation::UpdateGraphAlgorithm;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateGraphAlgorithm, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm {
        new_algorithm: base.graph.algorithm.clone(),
        new_algorithm_seed: base.graph.algorithm_seed.clone(),
    })]
}
//#endregion 🔖️Inverse
