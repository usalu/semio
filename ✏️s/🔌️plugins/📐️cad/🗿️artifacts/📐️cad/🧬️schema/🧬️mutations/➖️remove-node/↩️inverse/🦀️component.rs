//! ↩️ Inverse for `RemoveNode`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &CadSnapshot, node_id: &str) -> Vec<CadMutation> {
    base.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![CadMutation::AddNode { node: node.clone() }]).unwrap_or_default()
}
//#endregion 🔖️Inverse
