//! 🔺️ Sparse diff construction for the `replace-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::ReplaceBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ Whole-value swap of one row's non-identity content within the working-scene cache, then
/// re-mint a fresh content-addressed `table` child handle. Error `mutation.target-missing` if
/// absent, Warning `mutation.no-op` if the value is unchanged (both empty diff).
pub fn diff(payload: &ReplaceBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let mut records = crate::artifacts::program::program_benchmarks(base);
    let Some(existing) = records.iter_mut().find(|row| row.header.id == payload.benchmark_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No benchmark record exists with this id.", [payload.benchmark_record.header.id.0.clone()]);
    };
    if *existing == payload.benchmark_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This benchmark record already matches the requested value.").at([payload.benchmark_record.header.id.0.clone()])]);
    }
    *existing = payload.benchmark_record.clone();
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(crate::artifacts::program::benchmarks_child_from_records(&records)), ..Default::default() })
}
