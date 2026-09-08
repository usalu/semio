//! ↩️ Inverse (undo) construction for the `connect-adjacency` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗺️set-adjacency` per Wave C.

use crate::kernel::EntityId;
use crate::standards::v1::subsets::any::schema::normalize_pair;
use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ If the pair already existed, undo restores its prior full value; if this connect added a
/// brand-new edge, undo disconnects it by the id the connect used.
pub fn inverse(payload: &super::ConnectAdjacency, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    let (a, b) = normalize_pair(&payload.adjacency.element_a_id, &payload.adjacency.element_b_id);
    match base.adjacencies.iter().find(|row| row.element_a_id == a && row.element_b_id == b) {
        Some(existing) => vec![ProgramMutation::ConnectAdjacency(super::ConnectAdjacency { adjacency: existing.clone() })],
        None => {
            let id: EntityId = payload.adjacency.header.id.clone();
            vec![ProgramMutation::DisconnectAdjacency(super::super::disconnect_adjacency::DisconnectAdjacency { id })]
        }
    }
}
