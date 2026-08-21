//! ↩️ Inverse for `RenameNode` — recovers the pre-mutation `label` from `base`.
use super::mutation::RenameNode;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &RenameNode, base: &CadSnapshot) -> Vec<CadMutation> {
    base.nodes.iter().find(|node| node.id == payload.node_id).map(|node| vec![CadMutation::RenameNode(RenameNode { node_id: payload.node_id.clone(), new_label: node.label.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
