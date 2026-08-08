//! ↩️ Inverse for `RenameNode`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &CadSnapshot, node_id: &str, _label: &str) -> Vec<CadMutation> {
    base.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![CadMutation::RenameNode { node_id: node_id.into(), label: node.label.clone() }]).unwrap_or_default()
}
//#endregion 🔖️Inverse
