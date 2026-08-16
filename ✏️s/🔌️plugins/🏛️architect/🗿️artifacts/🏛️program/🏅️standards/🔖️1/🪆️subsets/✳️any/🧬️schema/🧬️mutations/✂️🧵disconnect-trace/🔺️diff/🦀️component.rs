//! 🔺️ Sparse diff construction for the `disconnect-trace` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧵traces` per Wave C.

use super::mutation::DisconnectTrace;
use crate::artifacts::program::diff::ProgramTracesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✂️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DisconnectTrace, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.traces.iter().any(|row| row.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No trace exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { traces: Some(ProgramTracesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
