//! ↩️ Inverse (undo) construction for the `disconnect-adjacency` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧹clear-adjacency` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo by reconnecting the captured edge. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DisconnectAdjacency, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.adjacencies.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::ConnectAdjacency { adjacency: base.adjacencies[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
