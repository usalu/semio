//! ↩️ Inverse (undo) construction for the `rename-benchmark-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏁benchmarks` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::RenameBenchmarkRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    let records = crate::artifacts::program::program_benchmarks(base);
    match records.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameBenchmarkRecord(super::mutation::RenameBenchmarkRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
