//! 🔺️ Sparse diff construction for the `rename-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::RenameBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Sets the target row's `header.name` within the working-scene cache, then re-mints a fresh
/// content-addressed `table` child handle. Missing target ⇒ the re-minted handle carries unchanged
/// rows (an effective no-op, same observable outcome as the former sparse-patch shape's no-op on
/// an unmatched id).
pub fn diff(payload: &RenameBenchmarkRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let mut records = crate::artifacts::program::program_benchmarks(base);
    if let Some(existing) = records.iter_mut().find(|row| row.header.id == payload.id) {
        existing.header.name = payload.new_name.clone();
    }
    ProgramDiff { benchmarks: Some(crate::artifacts::program::benchmarks_child_from_records(&records)), ..Default::default() }
}
