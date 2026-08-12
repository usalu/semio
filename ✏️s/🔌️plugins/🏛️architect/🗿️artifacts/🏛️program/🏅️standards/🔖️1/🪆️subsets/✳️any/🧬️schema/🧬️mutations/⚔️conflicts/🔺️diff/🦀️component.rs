//! 🔺️ Sparse diff construction for the `conflicts` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateConflict, DeleteConflict, RenameConflict, ReplaceConflict};
use crate::artifacts::program::diff::{ProgramConflictsDelta, ProgramConflictsPatchEntry};
use crate::artifacts::program::registers::ConflictPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.conflicts` on apply.
pub fn diff_create(payload: &CreateConflict, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { conflicts: Some(ProgramConflictsDelta { added: vec![payload.conflict.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteConflict, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { conflicts: Some(ProgramConflictsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameConflict, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ConflictPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { conflicts: Some(ProgramConflictsDelta { patched: vec![ProgramConflictsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceConflict, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.conflicts.iter().find(|row| row.header.id == payload.conflict.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.conflict).expect("diff_patch always produces a full patch");
    ProgramDiff { conflicts: Some(ProgramConflictsDelta { patched: vec![ProgramConflictsPatchEntry { id: payload.conflict.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
