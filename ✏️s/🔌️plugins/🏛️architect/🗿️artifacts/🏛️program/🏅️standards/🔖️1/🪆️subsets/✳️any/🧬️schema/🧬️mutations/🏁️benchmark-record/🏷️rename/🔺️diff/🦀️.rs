//! 🔺️ Sparse diff construction for the `rename-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::RenameBenchmarkRecord;
use crate::diff::{ProgramBenchmarksDelta, ProgramBenchmarksPatchEntry};
use crate::registers::BenchmarkRecordPatch;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `modified = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.benchmarks_payload.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No benchmark record exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This benchmark record already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = BenchmarkRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { modified: vec![ProgramBenchmarksPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
