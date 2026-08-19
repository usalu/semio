//! ↩️ Inverse (undo) construction for the `delete-benchmark-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏁benchmarks` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteBenchmarkRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    let records = crate::artifacts::program::program_benchmarks(base);
    match records.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateBenchmarkRecord(super::super::create_benchmark_record::mutation::CreateBenchmarkRecord { benchmark_record: existing.clone() })],
        None => Vec::new(),
    }
}
