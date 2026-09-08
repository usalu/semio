//! ↩️ Inverse for `ChangeNodeKind` — the OLD `nodeKind` looked up from BASE. Missing target ⇒
//! `Vec::new()`.

use crate::mutations::WiresMutation;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeNodeKind, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id).and_then(|node| node.get("nodeKind").and_then(|value| value.as_str()).map(str::to_string)) {
        Some(old_kind) => vec![crate::mutations::change_node_kind::change_node_kind(payload.node_id.clone(), old_kind)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
