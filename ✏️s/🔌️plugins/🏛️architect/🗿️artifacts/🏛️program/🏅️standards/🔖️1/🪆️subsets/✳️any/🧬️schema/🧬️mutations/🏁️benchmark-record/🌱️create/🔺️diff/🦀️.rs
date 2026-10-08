//! 🔺️ Sparse diff construction for the `create-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::CreateBenchmarkRecord;
use crate::diff::ProgramBenchmarksDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.benchmark_record.header.id;
    if base.benchmarks_payload.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A benchmark record already exists with this id.", [id.0.clone()]);
    }
    let length = base.benchmarks_payload.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the benchmark record list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.benchmarks_payload.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { added: vec![payload.benchmark_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
