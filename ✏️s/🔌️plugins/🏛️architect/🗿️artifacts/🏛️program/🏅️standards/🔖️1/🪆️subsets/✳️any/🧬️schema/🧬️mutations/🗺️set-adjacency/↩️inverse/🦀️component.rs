//! ↩️ Inverse (undo) construction for `ConnectAdjacency` — computed from captured pre-state.

use super::mutation::ConnectAdjacency;
use crate::artifacts::program::engine::adjacency::normalize_pair;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ If the pair already existed, undo restores its prior full value; if this connect added a
/// brand-new edge, undo disconnects it by the id the connect used.
pub fn inverse_connect(payload: &ConnectAdjacency, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    let (a, b) = normalize_pair(&payload.adjacency.element_a_id, &payload.adjacency.element_b_id);
    match base.adjacencies.iter().find(|row| row.element_a_id == a && row.element_b_id == b) {
        Some(existing) => vec![ProgramMutation::ConnectAdjacency(ConnectAdjacency { adjacency: existing.clone() })],
        None => {
            let id: EntityId = payload.adjacency.header.id.clone();
            vec![ProgramMutation::DisconnectAdjacency(super::super::clear_adjacency::mutation::DisconnectAdjacency { id })]
        }
    }
}
