//! ↩️ `change-graph-directed` — undo reconstructed from BASE state.

use crate::artifacts::equation::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeGraphDirected, base: &EquationSnapshot) -> Vec<EquationMutation> {
    vec![EquationMutation::ChangeGraphDirected(super::ChangeGraphDirected { new_directed: crate::artifacts::equation::equation_graph(base).directed })]
}
//#endregion 🔖️Inverse
