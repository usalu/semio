//! 🔺️ Sparse diff construction for the `connect-trace` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧵traces` per Wave C.

use super::ConnectTrace;
use crate::diff::ProgramTracesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔌️ Warning `mutation.no-op` if the trace already carries this exact value (empty diff); else
/// `added = [trace]` (plus `reordered` when `index` places it before the end; Error `mutation.index-out-of-range` past the end) if the id is new, else the trace is replaced under its own id: `removed = [id]`, `added = [trace]`, and `reordered`
/// (the base order) unless the trace was last. `from_id`/`to_id` are free-form cross-register references (any entity across any
/// collection) — endpoint-existence checking is not implemented here; see `📓️w3-d-architect-report.md`.
pub fn diff(payload: &ConnectTrace, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.trace.id;
    let Some(position) = base.traces.iter().position(|row| row.id == *id) else {
        let length = base.traces.len();
        let at = payload.index.unwrap_or(length);
        if at > length {
            return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the trace list.", [id.0.clone()]);
        }
        let reordered = (at < length).then(|| {
            let mut order: Vec<String> = base.traces.iter().map(|row| row.id.0.clone()).collect();
            order.insert(at, id.0.clone());
            order
        });
        return protocol::MutationOutcome::new(ProgramDiff { traces: Some(ProgramTracesDelta { added: vec![payload.trace.clone()], reordered, ..Default::default() }), ..Default::default() });
    };
    if base.traces[position] == payload.trace {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This trace already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.traces.len()).then(|| base.traces.iter().map(|row| row.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { traces: Some(ProgramTracesDelta { removed: vec![id.0.clone()], added: vec![payload.trace.clone()], reordered, ..Default::default() }), ..Default::default() })
}
