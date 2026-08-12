//! ↩️ Inverse (undo) construction for the `connect-trace` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧵traces` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ If the id already existed, undo restores its prior full value; otherwise undo disconnects
/// the newly-added edge.
pub fn inverse(payload: &super::mutation::ConnectTrace, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.traces.iter().find(|row| row.id == payload.trace.id) {
        Some(existing) => vec![ProgramMutation::ConnectTrace(super::mutation::ConnectTrace { trace: existing.clone() })],
        None => vec![ProgramMutation::DisconnectTrace(super::super::disconnect_trace::mutation::DisconnectTrace { id: payload.trace.id.clone() })],
    }
}
