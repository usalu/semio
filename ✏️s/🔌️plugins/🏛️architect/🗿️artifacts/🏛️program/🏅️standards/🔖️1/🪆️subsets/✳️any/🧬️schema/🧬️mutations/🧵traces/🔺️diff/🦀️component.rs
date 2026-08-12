//! 🔺️ Sparse diff construction for the `traces` mutation leaf.

use super::mutation::{ConnectTrace, DisconnectTrace};
use crate::artifacts::program::diff::ProgramTracesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔌️ `added = [trace]` if the id is new, else `patched = [{id, full patch}]`.
pub fn diff_connect(payload: &ConnectTrace, base: &ProgramSnapshot) -> ProgramDiff {
    match base.traces.iter().find(|row| row.id == payload.trace.id) {
        Some(existing) => {
            let patch = existing.diff_patch(&payload.trace).expect("diff_patch always produces a full patch");
            ProgramDiff { traces: Some(ProgramTracesDelta { patched: vec![crate::artifacts::program::diff::ProgramTracesPatchEntry { id: payload.trace.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
        }
        None => ProgramDiff { traces: Some(ProgramTracesDelta { added: vec![payload.trace.clone()], ..Default::default() }), ..Default::default() },
    }
}

/// ✂️ `removed = [id]`.
pub fn diff_disconnect(payload: &DisconnectTrace, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { traces: Some(ProgramTracesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
