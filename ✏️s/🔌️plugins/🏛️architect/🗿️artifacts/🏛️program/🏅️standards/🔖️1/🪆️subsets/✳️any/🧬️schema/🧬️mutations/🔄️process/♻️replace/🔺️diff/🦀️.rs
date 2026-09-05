//! 🔺️ Sparse diff construction for the `replace-process` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔄processes` per Wave C.

use super::ReplaceProcess;
use crate::artifacts::program::diff::{ProgramProcessesDelta, ProgramProcessesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceProcess, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.processes.iter().find(|row| row.header.id == payload.process.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No process exists with this id.", [payload.process.header.id.0.clone()]);
    };
    if existing == &payload.process {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This process already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.process).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { processes: Some(ProgramProcessesDelta { patched: vec![ProgramProcessesPatchEntry { id: payload.process.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
