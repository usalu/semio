//! ↩️ Inverse for `ConnectNodes` — disconnects by the edge's own id (already carried on the
//! payload, no BASE lookup needed). Missing id ⇒ `Vec::new()`.
use crate::artifacts::wires::schema::entity_id;
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ConnectNodes, _base: &WiresSnapshot) -> Vec<WiresMutation> {
    match entity_id(&payload.edge, "id") {
        Some(id) => vec![crate::artifacts::wires::mutations::disconnect_nodes::mutation::disconnect_nodes(id.to_string())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
