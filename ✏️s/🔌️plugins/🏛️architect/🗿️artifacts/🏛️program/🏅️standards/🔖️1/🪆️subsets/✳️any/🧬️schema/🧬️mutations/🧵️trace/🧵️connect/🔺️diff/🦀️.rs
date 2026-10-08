//! 🔺️ Sparse diff construction for the `connect-trace` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧵traces` per Wave C.

use super::ConnectTrace;
use crate::diff::ProgramTracesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔌️ Warning `mutation.no-op` if the trace already carries this exact value, Error `mutation.target-missing` if a new trace's `index` lies past the end (both empty diff);
/// else `inserted = [{index, trace}]` if the id is new (appended when `index` is absent), else the trace is replaced in place under its own id:
/// `removed = [{id, index}]` and `inserted = [{index, trace}]`. `from_id`/`to_id` are free-form cross-register references (any entity across any
/// collection) — endpoint-existence checking is not implemented here; see `📓️w3-d-architect-report.md`.
pub fn diff(payload: &ConnectTrace, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.trace.id;
    let Some(position) = base.traces.iter().position(|row| row.id == *id) else {
        let at = payload.index.unwrap_or(base.traces.len());
        if at > base.traces.len() {
            return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the trace list.", [id.0.clone()]);
        }
        return protocol::MutationOutcome::new(ProgramDiff { traces: Some(ProgramTracesDelta::insertion(at, payload.trace.clone())), ..Default::default() });
    };
    if base.traces[position] == payload.trace {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This trace already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramTracesDelta::removal(&base.traces, position);
    delta.absorb(ProgramTracesDelta::insertion(position, payload.trace.clone()));
    protocol::MutationOutcome::new(ProgramDiff { traces: Some(delta), ..Default::default() })
}
