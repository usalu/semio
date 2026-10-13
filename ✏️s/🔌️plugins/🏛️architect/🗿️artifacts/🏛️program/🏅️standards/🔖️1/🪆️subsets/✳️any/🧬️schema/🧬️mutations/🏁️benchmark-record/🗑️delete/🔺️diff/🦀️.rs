//! 🔺️ Sparse diff construction for the `delete-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::DeleteBenchmarkRecord;
use crate::standards::v1::subsets::any::schema::diff::ProgramBenchmarksDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff); else `removed = [{id, index}]` — `apply` re-derives the composed child handle from the remaining rows.
pub fn diff(payload: &DeleteBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.benchmarks_payload.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No benchmark record exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta::removal(&base.benchmarks_payload, position)), ..Default::default() })
}
