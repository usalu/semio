//! 🔺️ Sparse diff construction for the `rename-collaboration-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🤝collaboration` per Wave C.

use super::RenameCollaborationRecord;
use crate::artifacts::program::diff::{ProgramCollaborationDelta, ProgramCollaborationPatchEntry};
use crate::artifacts::program::registers::CollaborationRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameCollaborationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.collaboration.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No collaboration record exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This collaboration record already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = CollaborationRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { collaboration: Some(ProgramCollaborationDelta { patched: vec![ProgramCollaborationPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
