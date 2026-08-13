//! 🔺️ Sparse diff construction for the `create-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::CreateBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Reads the live `benchmarks` rows off the working-scene cache, appends the payload row, and
/// re-mints a fresh content-addressed `table` child handle — composed-child equivalent of the
/// former `added = [payload row]` sparse delta (`📓️migration-recipe.md` §3/§4).
pub fn diff(payload: &CreateBenchmarkRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let mut records = crate::artifacts::program::program_benchmarks(base);
    records.push(payload.benchmark_record.clone());
    ProgramDiff { benchmarks: Some(crate::artifacts::program::benchmarks_child_from_records(&records)), ..Default::default() }
}
