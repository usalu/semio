//! 🔺️ Sparse diff construction for the `elements` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateProgramElement, DeleteProgramElement, RenameProgramElement, ReplaceProgramElement};
use crate::artifacts::program::diff::{ProgramElementsDelta, ProgramElementsPatchEntry};
use crate::artifacts::program::registers::ProgramElementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.elements` on apply.
pub fn diff_create(payload: &CreateProgramElement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { elements: Some(ProgramElementsDelta { added: vec![payload.program_element.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteProgramElement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { elements: Some(ProgramElementsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameProgramElement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ProgramElementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { elements: Some(ProgramElementsDelta { patched: vec![ProgramElementsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceProgramElement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.elements.iter().find(|row| row.header.id == payload.program_element.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.program_element).expect("diff_patch always produces a full patch");
    ProgramDiff { elements: Some(ProgramElementsDelta { patched: vec![ProgramElementsPatchEntry { id: payload.program_element.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
