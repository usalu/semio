//! ↩️ `replace-graph` — undo reconstructed from BASE state (the whole prior graph).

use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceGraph, base: &EquationSnapshot) -> Vec<EquationMutation> {
    vec![EquationMutation::ReplaceGraph(super::ReplaceGraph { graph: crate::equation_graph(base) })]
}
//#endregion 🔖️Inverse
