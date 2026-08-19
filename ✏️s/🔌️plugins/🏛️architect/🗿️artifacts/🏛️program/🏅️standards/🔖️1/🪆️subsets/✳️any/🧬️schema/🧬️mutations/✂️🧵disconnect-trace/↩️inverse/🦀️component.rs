//! ↩️ Inverse (undo) construction for the `disconnect-trace` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧵traces` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo by reconnecting the captured edge. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DisconnectTrace, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.traces.iter().find(|row| row.id == payload.id) {
        Some(existing) => vec![ProgramMutation::ConnectTrace(super::super::connect_trace::mutation::ConnectTrace { trace: existing.clone() })],
        None => Vec::new(),
    }
}
