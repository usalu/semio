//! 🔺️ Sparse diff construction for the `replace-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::ReplaceBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ Whole-value swap of one row's non-identity content within the working-scene cache, then
/// re-mint a fresh content-addressed `table` child handle. Target absent from `base` ⇒ empty diff
/// (nothing to change) — same observable behavior as the former sparse-patch shape.
pub fn diff(payload: &ReplaceBenchmarkRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let mut records = crate::artifacts::program::program_benchmarks(base);
    let Some(existing) = records.iter_mut().find(|row| row.header.id == payload.benchmark_record.header.id) else {
        return ProgramDiff::default();
    };
    *existing = payload.benchmark_record.clone();
    ProgramDiff { benchmarks: Some(crate::artifacts::program::benchmarks_child_from_records(&records)), ..Default::default() }
}
