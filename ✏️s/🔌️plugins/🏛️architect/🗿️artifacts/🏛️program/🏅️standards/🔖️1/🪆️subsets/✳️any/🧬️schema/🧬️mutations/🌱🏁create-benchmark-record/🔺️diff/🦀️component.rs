//! 🔺️ Sparse diff construction for the `create-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::CreateBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramBenchmarksDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.benchmarks` on apply.
pub fn diff(payload: &CreateBenchmarkRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { added: vec![payload.benchmark_record.clone()], ..Default::default() }), ..Default::default() }
}
