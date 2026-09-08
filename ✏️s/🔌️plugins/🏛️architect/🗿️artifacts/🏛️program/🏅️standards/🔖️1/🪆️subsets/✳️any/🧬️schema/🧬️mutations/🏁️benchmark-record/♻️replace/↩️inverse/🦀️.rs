//! ↩️ Inverse (undo) construction for the `replace-benchmark-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏁benchmarks` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceBenchmarkRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    let records = crate::program_benchmarks(base);
    match records.iter().find(|row| row.header.id == payload.benchmark_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceBenchmarkRecord(super::ReplaceBenchmarkRecord { benchmark_record: existing.clone() })],
        None => Vec::new(),
    }
}
