//! ↩️ Inverse (undo) construction for the `connect-adjacency` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗺️set-adjacency` per Wave C.

use crate::standards::v1::subsets::any::schema::kernel::EntityId;
use crate::standards::v1::subsets::any::schema::kernel::normalize_pair;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ If the pair already existed, undo restores its prior full value; if this connect added a
/// brand-new edge, undo disconnects it by the id the connect used.
pub fn inverse<Op>(payload: &super::ConnectAdjacency, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_adjacency::ConnectAdjacency> + From<crate::standards::v1::subsets::any::schema::mutations::disconnect_adjacency::DisconnectAdjacency>,
{
    Ok((|| {
    let (a, b) = normalize_pair(&payload.adjacency.element_a_id, &payload.adjacency.element_b_id);
    match base.adjacencies.iter().find(|row| row.element_a_id == a && row.element_b_id == b) {
        Some(existing) => vec![Op::from(super::ConnectAdjacency { adjacency: existing.clone(), index: None })],
        None => {
            let id: EntityId = payload.adjacency.header.id.clone();
            vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::disconnect_adjacency::DisconnectAdjacency { id })]
        }
    }

    })())
}
