//! ↩️ Inverse (undo) construction for the `create-benchmark-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏁benchmarks` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::CreateBenchmarkRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteBenchmarkRecord(super::super::delete_benchmark_record::DeleteBenchmarkRecord { id: payload.benchmark_record.header.id.clone() })]
}
