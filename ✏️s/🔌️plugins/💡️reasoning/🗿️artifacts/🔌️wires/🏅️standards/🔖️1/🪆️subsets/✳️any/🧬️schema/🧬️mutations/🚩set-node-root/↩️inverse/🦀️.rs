//! ↩️ Inverse for `SetNodeRoot` — the OLD `root` looked up from BASE (absent ⇒ `false`, the
//! `NodeDsl` field's own default). Missing target ⇒ `Vec::new()`.

use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::SetNodeRoot, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id) {
        Some(node) => {
            let old_root = node.get("root").and_then(|value| value.as_bool()).unwrap_or(false);
            vec![crate::artifacts::wires::mutations::set_node_root::set_node_root(payload.node_id.clone(), old_root)]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
