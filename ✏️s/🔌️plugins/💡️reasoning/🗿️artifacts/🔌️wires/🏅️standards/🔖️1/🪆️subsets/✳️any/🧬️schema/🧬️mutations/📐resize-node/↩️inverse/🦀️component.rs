//! ↩️ Inverse for `ResizeNode` — captures the OLD extent from BASE for exactly the fields the
//! payload touched (untouched fields stay `None`, meaning "leave alone"). Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ResizeNode, base: &WiresSnapshot) -> Vec<WiresMutation> {
    let Some(node) = find_board_node(base, &payload.node_id) else { return Vec::new() };
    let old_radius = payload.new_radius.and(node.get("radius").and_then(|value| value.as_f64()));
    let old_width = payload.new_width.and(node.get("width").and_then(|value| value.as_f64()));
    let old_height = payload.new_height.and(node.get("height").and_then(|value| value.as_f64()));
    vec![crate::artifacts::wires::mutations::resize_node::mutation::resize_node(payload.node_id.clone(), old_radius, old_width, old_height)]
}
//#endregion 🔖️Inverse
