//! ↩️ Inverse for `CreateNode` — deletes the just-created node by its own id (no BASE lookup
//! needed: the id is already carried on the payload). Missing id ⇒ `Vec::new()`.

use crate::mutations::WiresMutation;
use crate::schema::entity_id;
use crate::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateNode, _base: &WiresSnapshot) -> Vec<WiresMutation> {
    match entity_id(&payload.node, "id") {
        Some(id) => vec![crate::mutations::delete_node::delete_node(id.to_string())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
