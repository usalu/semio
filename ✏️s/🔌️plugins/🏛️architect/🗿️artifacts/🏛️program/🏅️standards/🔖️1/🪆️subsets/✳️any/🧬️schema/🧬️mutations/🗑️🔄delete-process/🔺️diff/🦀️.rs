//! 🔺️ Sparse diff construction for the `delete-process` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔄processes` per Wave C.

use super::DeleteProcess;
use crate::artifacts::program::diff::ProgramProcessesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteProcess, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.processes.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No process exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { processes: Some(ProgramProcessesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
