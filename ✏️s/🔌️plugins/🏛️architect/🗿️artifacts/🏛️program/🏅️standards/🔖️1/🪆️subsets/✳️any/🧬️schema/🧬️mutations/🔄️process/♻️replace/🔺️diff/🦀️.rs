//! 🔺️ Sparse diff construction for the `replace-process` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔄processes` per Wave C.

use super::ReplaceProcess;
use crate::diff::ProgramProcessesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceProcess, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.process.header.id;
    let Some(position) = base.processes.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No process exists with this id.", [id.0.clone()]);
    };
    if base.processes[position] == payload.process {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This process already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramProcessesDelta::removal(&base.processes, position);
    delta.absorb(ProgramProcessesDelta::insertion(position, payload.process.clone()));
    protocol::MutationOutcome::new(ProgramDiff { processes: Some(delta), ..Default::default() })
}
