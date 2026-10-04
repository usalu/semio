//! ↩️ `replace-graph` — undo reconstructed from BASE state (the whole prior graph).

use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceGraph, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![EquationMutation::ReplaceGraph(super::ReplaceGraph { graph: base.graph.clone() })]

    })())
}
//#endregion 🔖️Inverse
