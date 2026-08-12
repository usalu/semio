//! 🔺️ Sparse diff construction for the `delete-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::DeleteBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramBenchmarksDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteBenchmarkRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
