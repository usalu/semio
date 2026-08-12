//! 🔺️ Sparse diff construction for the `changes` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateChangeRecord, DeleteChangeRecord, RenameChangeRecord, ReplaceChangeRecord};
use crate::artifacts::program::diff::{ProgramChangesDelta, ProgramChangesPatchEntry};
use crate::artifacts::program::registers::ChangeRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.changes` on apply.
pub fn diff_create(payload: &CreateChangeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { changes: Some(ProgramChangesDelta { added: vec![payload.change_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteChangeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { changes: Some(ProgramChangesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameChangeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ChangeRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { changes: Some(ProgramChangesDelta { patched: vec![ProgramChangesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceChangeRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.changes.iter().find(|row| row.header.id == payload.change_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.change_record).expect("diff_patch always produces a full patch");
    ProgramDiff { changes: Some(ProgramChangesDelta { patched: vec![ProgramChangesPatchEntry { id: payload.change_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
