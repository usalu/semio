//! ↩️ `create-node` — undo is `delete-node`, unless `base` already had this id (then `create` was
//! a no-op and there's nothing to undo).

use crate::standards::v1::subsets::graph::schema::mutations::delete_node;
use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::CreateNode, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::delete_node::DeleteNode>,
{
    Ok((|| {
    if base.graph.nodes.iter().any(|node| node.id == payload.id) {
        return Vec::new();
    }
    vec![Op::from(delete_node::DeleteNode { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
