//! 🔺️ Sparse diff construction for the `replace-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::ReplaceBenchmarkRecord;
use crate::diff::ProgramBenchmarksDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.benchmark_record.header.id;
    let Some(position) = base.benchmarks_payload.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No benchmark record exists with this id.", [id.0.clone()]);
    };
    if base.benchmarks_payload[position] == payload.benchmark_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This benchmark record already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.benchmarks_payload.len()).then(|| base.benchmarks_payload.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(ProgramBenchmarksDelta { removed: vec![id.0.clone()], added: vec![payload.benchmark_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
