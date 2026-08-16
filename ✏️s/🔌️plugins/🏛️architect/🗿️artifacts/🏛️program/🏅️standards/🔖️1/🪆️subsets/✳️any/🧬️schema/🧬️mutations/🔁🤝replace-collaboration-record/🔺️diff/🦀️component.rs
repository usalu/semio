//! 🔺️ Sparse diff construction for the `replace-collaboration-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🤝collaboration` per Wave C.

use super::mutation::ReplaceCollaborationRecord;
use crate::artifacts::program::diff::{ProgramCollaborationDelta, ProgramCollaborationPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub fn diff(payload: &ReplaceCollaborationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.collaboration.iter().find(|row| row.header.id == payload.collaboration_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No collaboration record exists with this id.", [payload.collaboration_record.header.id.0.clone()]);
    };
    if existing == &payload.collaboration_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This collaboration record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.collaboration_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { collaboration: Some(ProgramCollaborationDelta { patched: vec![ProgramCollaborationPatchEntry { id: payload.collaboration_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
