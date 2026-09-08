//! ↩️ Inverse for `ConnectNodes` — disconnects by the edge's own id (already carried on the
//! payload, no BASE lookup needed). Missing id ⇒ `Vec::new()`.

use crate::mutations::WiresMutation;
use crate::schema::entity_id;
use crate::WiresSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ConnectNodes, _base: &WiresSnapshot) -> Vec<WiresMutation> {
    match entity_id(&payload.edge, "id") {
        Some(id) => vec![crate::mutations::disconnect_nodes::disconnect_nodes(id.to_string())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
