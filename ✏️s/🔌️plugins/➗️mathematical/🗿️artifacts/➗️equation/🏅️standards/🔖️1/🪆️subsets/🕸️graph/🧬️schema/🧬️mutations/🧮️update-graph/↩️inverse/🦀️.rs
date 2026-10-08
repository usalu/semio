//! ↩️ `update-graph-algorithm` — undo reconstructed from BASE state.

use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateGraphAlgorithm, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let graph = &base.graph;
    vec![EquationMutation::UpdateGraphAlgorithm(super::UpdateGraphAlgorithm { new_algorithm: graph.algorithm.clone(), new_algorithm_seed: graph.algorithm_seed.clone() })]

    })())
}
//#endregion 🔖️Inverse
