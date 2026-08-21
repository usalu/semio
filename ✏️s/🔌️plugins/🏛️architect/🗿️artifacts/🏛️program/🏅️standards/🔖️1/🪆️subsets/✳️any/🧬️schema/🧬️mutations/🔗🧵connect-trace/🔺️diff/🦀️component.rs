//! 🔺️ Sparse diff construction for the `connect-trace` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧵traces` per Wave C.

use super::mutation::ConnectTrace;
use crate::artifacts::program::diff::ProgramTracesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔌️ Warning `mutation.no-op` if the trace already carries this exact value (empty diff); else
/// `added = [trace]` if the id is new, else `patched = [{id, full patch}]`. `from_id`/`to_id` are
/// free-form cross-register references (any entity across any collection) — endpoint-existence
/// checking is not implemented here; see `📓️w3-d-architect-report.md`.
pub async fn diff(payload: &ConnectTrace, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    match base.traces.iter().find(|row| row.id == payload.trace.id) {
        Some(existing) => {
            if existing == &payload.trace {
                return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This trace already matches the requested value.").at([existing.id.0.clone()])]);
            }
            let patch = existing.diff_patch(&payload.trace).expect("diff_patch always produces a full patch");
            protocol::MutationOutcome::new(ProgramDiff {
                traces: Some(ProgramTracesDelta { patched: vec![crate::artifacts::program::diff::ProgramTracesPatchEntry { id: payload.trace.id.0.clone(), patch }], ..Default::default() }),
                ..Default::default()
            })
        }
        None => protocol::MutationOutcome::new(ProgramDiff { traces: Some(ProgramTracesDelta { added: vec![payload.trace.clone()], ..Default::default() }), ..Default::default() }),
    }
}
