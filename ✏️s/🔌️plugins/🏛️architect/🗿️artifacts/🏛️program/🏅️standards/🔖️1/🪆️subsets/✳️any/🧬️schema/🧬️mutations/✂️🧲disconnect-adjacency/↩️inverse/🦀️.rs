//! ↩️ Inverse (undo) construction for the `disconnect-adjacency` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧹clear-adjacency` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo by reconnecting the captured edge. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DisconnectAdjacency, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.adjacencies.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::ConnectAdjacency { adjacency: existing.clone() })],
        None => Vec::new(),
    }
}
