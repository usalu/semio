//! 🔺️ Sparse diff construction for the `create-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::CreateBenchmarkRecord;
use crate::diff::ProgramBenchmarksDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.benchmark_record.header.id;
    if base.benchmarks_payload.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A benchmark record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.benchmarks_payload.len());
    if at > base.benchmarks_payload.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the benchmark record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta::insertion(at, payload.benchmark_record.clone())), ..Default::default() })
}
