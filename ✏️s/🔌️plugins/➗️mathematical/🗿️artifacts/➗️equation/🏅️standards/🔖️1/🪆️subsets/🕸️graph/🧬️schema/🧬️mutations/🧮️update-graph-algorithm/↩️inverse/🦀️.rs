//! ↩️ `update-graph-algorithm` — undo reconstructed from BASE state.

use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &super::UpdateGraphAlgorithm, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm::UpdateGraphAlgorithm>,
{
    Ok((|| {
    let graph = &base.graph;
    vec![Op::from(super::UpdateGraphAlgorithm { new_algorithm: graph.algorithm.clone(), new_algorithm_seed: graph.algorithm_seed.clone() })]

    })())
}
//#endregion 🔖️Inverse
