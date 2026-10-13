//! 🔺️ Sparse diff construction for the `disconnect-trace` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧵traces` per Wave C.

use super::DisconnectTrace;
use crate::standards::v1::subsets::any::schema::diff::ProgramTracesDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ✂️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DisconnectTrace, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.traces.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No trace exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { traces: Some(ProgramTracesDelta::removal(&base.traces, position)), ..Default::default() })
}
