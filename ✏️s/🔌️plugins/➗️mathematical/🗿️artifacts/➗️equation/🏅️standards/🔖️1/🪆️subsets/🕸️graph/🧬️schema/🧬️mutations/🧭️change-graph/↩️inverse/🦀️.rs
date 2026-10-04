//! ↩️ `change-graph-directed` — undo reconstructed from BASE state.

use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeGraphDirected, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![EquationMutation::ChangeGraphDirected(super::ChangeGraphDirected { new_directed: base.graph.directed })]

    })())
}
//#endregion 🔖️Inverse
