//! 🔺️ Sparse diff construction for the `create-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::CreateBenchmarkRecord;
use crate::diff::ProgramBenchmarksDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff); else `added = [payload row]` — `apply` re-derives the composed child handle from the rows.
pub fn diff(payload: &CreateBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.benchmark_record.header.id;
    if base.benchmarks_payload.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A benchmark record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { added: vec![payload.benchmark_record.clone()], ..Default::default() }), ..Default::default() })
}
