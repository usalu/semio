//! ↩️ `connect-nodes` — undo is `disconnect-nodes`, unless `base` already had this edge id (then
//! `connect` was a no-op, matching `create-node`'s duplicate-id handling).

use crate::standards::v1::subsets::graph::schema::mutations::disconnect_nodes;
use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ConnectNodes, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::disconnect_nodes::DisconnectNodes>,
{
    Ok((|| {
    if base.graph.edges.iter().any(|edge| edge.id == payload.id) {
        return Vec::new();
    }
    vec![Op::from(disconnect_nodes::DisconnectNodes { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
