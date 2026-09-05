//! ↩️ `replace-graph` — undo reconstructed from BASE state (the whole prior graph).

use crate::artifacts::equation::{equation_graph, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ReplaceGraph, base: &EquationSnapshot) -> Vec<EquationMutation> {
    vec![EquationMutation::ReplaceGraph(super::ReplaceGraph { graph: crate::artifacts::equation::equation_graph(base) })]
}
//#endregion 🔖️Inverse
