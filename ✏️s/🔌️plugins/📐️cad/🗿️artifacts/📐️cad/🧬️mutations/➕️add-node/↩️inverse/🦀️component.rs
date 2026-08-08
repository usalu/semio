//! ↩️ Inverse for `AddNode`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadNode, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(_base: &CadSnapshot, node: &CadNode) -> Vec<CadMutation> {
    vec![CadMutation::RemoveNode { node_id: node.id.clone() }]
}
//#endregion 🔖️Inverse
