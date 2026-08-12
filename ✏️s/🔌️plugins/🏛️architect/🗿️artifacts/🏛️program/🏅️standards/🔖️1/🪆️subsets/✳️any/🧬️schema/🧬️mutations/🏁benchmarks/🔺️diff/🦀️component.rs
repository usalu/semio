//! 🔺️ Sparse diff construction for the `benchmarks` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateBenchmarkRecord, DeleteBenchmarkRecord, RenameBenchmarkRecord, ReplaceBenchmarkRecord};
use crate::artifacts::program::diff::{ProgramBenchmarksDelta, ProgramBenchmarksPatchEntry};
use crate::artifacts::program::registers::BenchmarkRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.benchmarks` on apply.
pub fn diff_create(payload: &CreateBenchmarkRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { added: vec![payload.benchmark_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteBenchmarkRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameBenchmarkRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = BenchmarkRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { patched: vec![ProgramBenchmarksPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceBenchmarkRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.benchmarks.iter().find(|row| row.header.id == payload.benchmark_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.benchmark_record).expect("diff_patch always produces a full patch");
    ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { patched: vec![ProgramBenchmarksPatchEntry { id: payload.benchmark_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
