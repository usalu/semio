//! 🔺️ Sparse diff construction for the `replace-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::mutation::ReplaceBenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramBenchmarksDelta, ProgramBenchmarksPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceBenchmarkRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.benchmarks.iter().find(|row| row.header.id == payload.benchmark_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.benchmark_record).expect("diff_patch always produces a full patch");
    ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { patched: vec![ProgramBenchmarksPatchEntry { id: payload.benchmark_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
