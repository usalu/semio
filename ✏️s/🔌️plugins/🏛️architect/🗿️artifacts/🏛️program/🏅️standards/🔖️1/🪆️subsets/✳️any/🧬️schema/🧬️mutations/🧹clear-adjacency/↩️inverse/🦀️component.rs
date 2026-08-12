//! ↩️ Inverse (undo) construction for `DisconnectAdjacency` — computed from captured pre-state.

use super::mutation::DisconnectAdjacency;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo by reconnecting the captured edge. Missing target ⇒ nothing to undo.
pub fn inverse_disconnect(payload: &DisconnectAdjacency, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.adjacencies.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::ConnectAdjacency(super::super::set_adjacency::mutation::ConnectAdjacency { adjacency: existing.clone() })],
        None => Vec::new(),
    }
}
