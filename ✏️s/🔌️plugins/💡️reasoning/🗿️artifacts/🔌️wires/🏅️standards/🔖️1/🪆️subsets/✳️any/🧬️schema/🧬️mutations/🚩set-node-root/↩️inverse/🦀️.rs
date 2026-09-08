//! ↩️ Inverse for `SetNodeRoot` — the OLD `root` looked up from BASE (absent ⇒ `false`, the
//! `NodeDsl` field's own default). Missing target ⇒ `Vec::new()`.

use crate::mutations::WiresMutation;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::SetNodeRoot, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id) {
        Some(node) => {
            let old_root = node.get("root").and_then(|value| value.as_bool()).unwrap_or(false);
            vec![crate::mutations::set_node_root::set_node_root(payload.node_id.clone(), old_root)]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
