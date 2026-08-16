//! 🔺️ Sparse diff construction for the `create-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::CreateBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Reads the live `benchmarks` rows off the working-scene cache; Fatal `mutation.duplicate-id`
/// if the id already exists (empty diff); else appends the payload row and re-mints a fresh
/// content-addressed `table` child handle — composed-child equivalent of the former
/// `added = [payload row]` sparse delta (`📓️migration-recipe.md` §3/§4).
pub fn diff(payload: &CreateBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let mut records = crate::artifacts::program::program_benchmarks(base);
    if records.iter().any(|row| row.header.id == payload.benchmark_record.header.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A benchmark record already exists with this id.", [payload.benchmark_record.header.id.0.clone()]);
    }
    records.push(payload.benchmark_record.clone());
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(crate::artifacts::program::benchmarks_child_from_records(&records)), ..Default::default() })
}
