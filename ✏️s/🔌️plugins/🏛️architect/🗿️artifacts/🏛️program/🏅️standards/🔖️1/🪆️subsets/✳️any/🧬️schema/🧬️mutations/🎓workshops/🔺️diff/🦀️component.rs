//! 🔺️ Sparse diff construction for the `workshops` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateWorkshop, DeleteWorkshop, RenameWorkshop, ReplaceWorkshop};
use crate::artifacts::program::diff::{ProgramWorkshopsDelta, ProgramWorkshopsPatchEntry};
use crate::artifacts::program::registers::WorkshopPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.workshops` on apply.
pub fn diff_create(payload: &CreateWorkshop, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { workshops: Some(ProgramWorkshopsDelta { added: vec![payload.workshop.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteWorkshop, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { workshops: Some(ProgramWorkshopsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameWorkshop, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = WorkshopPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { workshops: Some(ProgramWorkshopsDelta { patched: vec![ProgramWorkshopsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceWorkshop, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.workshops.iter().find(|row| row.header.id == payload.workshop.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.workshop).expect("diff_patch always produces a full patch");
    ProgramDiff { workshops: Some(ProgramWorkshopsDelta { patched: vec![ProgramWorkshopsPatchEntry { id: payload.workshop.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
