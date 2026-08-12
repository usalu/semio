//! ↩️ Inverse (undo) construction for the `benchmarks` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateBenchmarkRecord, DeleteBenchmarkRecord, RenameBenchmarkRecord, ReplaceBenchmarkRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateBenchmarkRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteBenchmarkRecord(DeleteBenchmarkRecord { id: payload.benchmark_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteBenchmarkRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.benchmarks.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateBenchmarkRecord(CreateBenchmarkRecord { benchmark_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameBenchmarkRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.benchmarks.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameBenchmarkRecord(RenameBenchmarkRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceBenchmarkRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.benchmarks.iter().find(|row| row.header.id == payload.benchmark_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceBenchmarkRecord(ReplaceBenchmarkRecord { benchmark_record: existing.clone() })],
        None => Vec::new(),
    }
}
