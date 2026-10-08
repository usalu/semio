//! 🔺️ Sparse diff construction for the `replace-benchmark-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏁benchmarks` per Wave C.

use super::ReplaceBenchmarkRecord;
use crate::diff::ProgramBenchmarksDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceBenchmarkRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.benchmark_record.header.id;
    let Some(position) = base.benchmarks_payload.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No benchmark record exists with this id.", [id.0.clone()]);
    };
    if base.benchmarks_payload[position] == payload.benchmark_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This benchmark record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramBenchmarksDelta::removal(&base.benchmarks_payload, position);
    delta.absorb(ProgramBenchmarksDelta::insertion(position, payload.benchmark_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { benchmarks: Some(delta), ..Default::default() })
}
