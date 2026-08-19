//! ↩️ Inverse for `EditNodeText` — the OLD `text` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::EditNodeText, base: &WiresSnapshot) -> Vec<WiresMutation> {
    match find_board_node(base, &payload.node_id).and_then(|node| node.get("text").and_then(|value| value.as_str()).map(str::to_string)) {
        Some(old_text) => vec![crate::artifacts::wires::mutations::edit_node_text::mutation::edit_node_text(payload.node_id.clone(), old_text)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
