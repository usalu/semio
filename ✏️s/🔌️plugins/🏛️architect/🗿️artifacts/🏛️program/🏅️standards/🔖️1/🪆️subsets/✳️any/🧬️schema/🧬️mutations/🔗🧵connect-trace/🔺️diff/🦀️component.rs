//! 🔺️ Sparse diff construction for the `connect-trace` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧵traces` per Wave C.

use super::mutation::ConnectTrace;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::ProgramTracesDelta;

/// 🔌️ `added = [trace]` if the id is new, else `patched = [{id, full patch}]`.
pub fn diff(payload: &ConnectTrace, base: &ProgramSnapshot) -> ProgramDiff {
    match base.traces.iter().find(|row| row.id == payload.trace.id) {
        Some(existing) => {
            let patch = existing.diff_patch(&payload.trace).expect("diff_patch always produces a full patch");
            ProgramDiff { traces: Some(ProgramTracesDelta { patched: vec![crate::artifacts::program::diff::ProgramTracesPatchEntry { id: payload.trace.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
        }
        None => ProgramDiff { traces: Some(ProgramTracesDelta { added: vec![payload.trace.clone()], ..Default::default() }), ..Default::default() },
    }
}
