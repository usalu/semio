//! 🔺️ Sparse diff construction for the `delete-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::DeleteBenchmarkRecord;
use crate::diff::ProgramBenchmarksDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff); else `removed = [id]` — `apply` re-derives the composed child handle from the remaining rows.
pub fn diff(payload: &DeleteBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.benchmarks_payload.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No benchmark record exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
