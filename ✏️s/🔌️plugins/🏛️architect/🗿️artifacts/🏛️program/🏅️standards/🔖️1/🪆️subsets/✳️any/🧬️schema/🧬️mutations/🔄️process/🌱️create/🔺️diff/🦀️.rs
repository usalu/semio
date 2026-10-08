//! 🔺️ Sparse diff construction for the `create-process` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔄processes` per Wave C.

use super::CreateProcess;
use crate::diff::ProgramProcessesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateProcess, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.process.header.id;
    if base.processes.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A process already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.processes.len());
    if at > base.processes.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the process list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { processes: Some(ProgramProcessesDelta::insertion(at, payload.process.clone())), ..Default::default() })
}
