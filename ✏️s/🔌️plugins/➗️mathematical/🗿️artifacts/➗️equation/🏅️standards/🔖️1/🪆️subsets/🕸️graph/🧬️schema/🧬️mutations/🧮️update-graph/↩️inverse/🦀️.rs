//! ↩️ `update-graph-algorithm` — undo reconstructed from BASE state.

use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateGraphAlgorithm, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let graph = base.graph.clone();
    vec![EquationMutation::UpdateGraphAlgorithm(super::UpdateGraphAlgorithm { new_algorithm: graph.algorithm, new_algorithm_seed: graph.algorithm_seed })]

    })())
}
//#endregion 🔖️Inverse
