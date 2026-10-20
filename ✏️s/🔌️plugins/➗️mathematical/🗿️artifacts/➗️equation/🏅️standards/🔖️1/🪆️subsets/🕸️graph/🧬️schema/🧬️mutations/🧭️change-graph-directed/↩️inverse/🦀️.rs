//! ↩️ `change-graph-directed` — undo reconstructed from BASE state.

use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &super::ChangeGraphDirected, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::change_graph_directed::ChangeGraphDirected>,
{
    Ok((|| {
    vec![Op::from(super::ChangeGraphDirected { new_directed: base.graph.directed })]

    })())
}
//#endregion 🔖️Inverse
