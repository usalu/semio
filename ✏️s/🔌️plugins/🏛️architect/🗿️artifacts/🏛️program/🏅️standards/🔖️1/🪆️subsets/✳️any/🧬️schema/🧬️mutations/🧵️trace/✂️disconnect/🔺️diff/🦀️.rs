//! 🔺️ Sparse diff construction for the `disconnect-trace` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧵traces` per Wave C.

use super::DisconnectTrace;
use crate::diff::ProgramTracesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// ✂️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DisconnectTrace, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.traces.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No trace exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { traces: Some(ProgramTracesDelta::removal(&base.traces, position)), ..Default::default() })
}
