//! 🔺️ Sparse diff construction for the `rename-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::RenameBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramBenchmarksDelta, ProgramBenchmarksPatchEntry};
use crate::artifacts::program::registers::BenchmarkRecordPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameBenchmarkRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = BenchmarkRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { patched: vec![ProgramBenchmarksPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
