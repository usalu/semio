//! ↩️ Inverse for `EditNodeText` — the OLD `text` looked up from BASE. Missing target ⇒
//! `Vec::new()`.

use crate::mutations::WiresMutation;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::EditNodeText, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id).and_then(|node| node.get("text").and_then(|value| value.as_str()).map(str::to_string)) {
        Some(old_text) => vec![crate::mutations::edit_node_text::edit_node_text(payload.node_id.clone(), old_text)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
