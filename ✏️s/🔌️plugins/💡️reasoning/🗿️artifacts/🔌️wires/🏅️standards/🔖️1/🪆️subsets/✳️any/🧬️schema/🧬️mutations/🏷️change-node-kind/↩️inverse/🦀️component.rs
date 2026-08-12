//! ↩️ Inverse for `ChangeNodeKind` — the OLD `nodeKind` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeNodeKind, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id).and_then(|node| node.get("nodeKind")).and_then(|value| value.as_str()) {
        Some(old_kind) => vec![crate::artifacts::wires::mutations::change_node_kind::mutation::change_node_kind(payload.node_id.clone(), old_kind.to_string())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
