//! ↩️ Inverse for `CreateNode` — deletes the just-created node by its own id (no BASE lookup
//! needed: the id is already carried on the payload). Missing id ⇒ `Vec::new()`.
use crate::artifacts::wires::schema::entity_id;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateNode, _base: &WiresSnapshot) -> Vec<WiresMutation> {
    match entity_id(&payload.node, "id") {
        Some(id) => vec![crate::artifacts::wires::mutations::delete_node::mutation::delete_node(id.to_string())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
