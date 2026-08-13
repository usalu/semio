//! ↩️ Inverse (undo) construction for the `replace-benchmark-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏁benchmarks` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceBenchmarkRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    let records = crate::artifacts::program::program_benchmarks(base);
    match records.iter().find(|row| row.header.id == payload.benchmark_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceBenchmarkRecord(super::mutation::ReplaceBenchmarkRecord { benchmark_record: existing.clone() })],
        None => Vec::new(),
    }
}
