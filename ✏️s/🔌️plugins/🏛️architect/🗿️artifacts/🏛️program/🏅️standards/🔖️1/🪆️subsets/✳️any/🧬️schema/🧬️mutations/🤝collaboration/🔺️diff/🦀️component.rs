//! 🔺️ Sparse diff construction for the `collaboration` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateCollaborationRecord, DeleteCollaborationRecord, RenameCollaborationRecord, ReplaceCollaborationRecord};
use crate::artifacts::program::diff::{ProgramCollaborationDelta, ProgramCollaborationPatchEntry};
use crate::artifacts::program::registers::CollaborationRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.collaboration` on apply.
pub fn diff_create(payload: &CreateCollaborationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { collaboration: Some(ProgramCollaborationDelta { added: vec![payload.collaboration_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteCollaborationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { collaboration: Some(ProgramCollaborationDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameCollaborationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = CollaborationRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { collaboration: Some(ProgramCollaborationDelta { patched: vec![ProgramCollaborationPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceCollaborationRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.collaboration.iter().find(|row| row.header.id == payload.collaboration_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.collaboration_record).expect("diff_patch always produces a full patch");
    ProgramDiff { collaboration: Some(ProgramCollaborationDelta { patched: vec![ProgramCollaborationPatchEntry { id: payload.collaboration_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
