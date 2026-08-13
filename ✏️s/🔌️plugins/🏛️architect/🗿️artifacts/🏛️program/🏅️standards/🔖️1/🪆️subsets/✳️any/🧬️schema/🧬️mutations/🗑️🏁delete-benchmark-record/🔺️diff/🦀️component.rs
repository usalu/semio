//! 🔺️ Sparse diff construction for the `delete-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::DeleteBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Removes the target row from the working-scene cache, then re-mints a fresh
/// content-addressed `table` child handle over the remaining rows.
pub fn diff(payload: &DeleteBenchmarkRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let mut records = crate::artifacts::program::program_benchmarks(base);
    records.retain(|row| row.header.id != payload.id);
    ProgramDiff { benchmarks: Some(crate::artifacts::program::benchmarks_child_from_records(&records)), ..Default::default() }
}
