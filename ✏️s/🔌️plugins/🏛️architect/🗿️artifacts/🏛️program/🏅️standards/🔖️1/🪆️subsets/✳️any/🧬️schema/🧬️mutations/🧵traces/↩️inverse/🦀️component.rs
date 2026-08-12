//! ↩️ Inverse (undo) construction for the `traces` mutation leaf — computed from captured
//! pre-state.

use super::mutation::{ConnectTrace, DisconnectTrace};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ If the id already existed, undo restores its prior full value; otherwise undo disconnects
/// the newly-added edge.
pub fn inverse_connect(payload: &ConnectTrace, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.traces.iter().find(|row| row.id == payload.trace.id) {
        Some(existing) => vec![ProgramMutation::ConnectTrace(ConnectTrace { trace: existing.clone() })],
        None => vec![ProgramMutation::DisconnectTrace(DisconnectTrace { id: payload.trace.id.clone() })],
    }
}

/// ↩️ Undo by reconnecting the captured edge. Missing target ⇒ nothing to undo.
pub fn inverse_disconnect(payload: &DisconnectTrace, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.traces.iter().find(|row| row.id == payload.id) {
        Some(existing) => vec![ProgramMutation::ConnectTrace(ConnectTrace { trace: existing.clone() })],
        None => Vec::new(),
    }
}
