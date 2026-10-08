//! ↩️ Inverse (undo) construction for the `disconnect-trace` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧵traces` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo by reconnecting the captured edge. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DisconnectTrace, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.traces.iter().position(|row| row.id == payload.id) {
        Some(position) => vec![ProgramMutation::ConnectTrace(super::super::connect_trace::ConnectTrace { trace: base.traces[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
