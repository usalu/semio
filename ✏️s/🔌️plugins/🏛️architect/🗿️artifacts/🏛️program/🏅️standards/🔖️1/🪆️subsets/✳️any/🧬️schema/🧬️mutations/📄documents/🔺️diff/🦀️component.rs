//! 🔺️ Sparse diff construction for the `documents` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateDocument, DeleteDocument, RenameDocument, ReplaceDocument};
use crate::artifacts::program::diff::{ProgramArtifactsDelta, ProgramArtifactsPatchEntry};
use crate::artifacts::program::registers::ArtifactRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.artifacts` on apply.
pub fn diff_create(payload: &CreateDocument, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { documents: Some(ProgramArtifactsDelta { added: vec![payload.document.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteDocument, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { documents: Some(ProgramArtifactsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameDocument, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ArtifactRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { documents: Some(ProgramArtifactsDelta { patched: vec![ProgramArtifactsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceDocument, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.artifacts.iter().find(|row| row.header.id == payload.document.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.document).expect("diff_patch always produces a full patch");
    ProgramDiff { documents: Some(ProgramArtifactsDelta { patched: vec![ProgramArtifactsPatchEntry { id: payload.document.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
