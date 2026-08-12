//! 🔺️ Sparse diff construction for the `information` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateInformationRequirement, DeleteInformationRequirement, RenameInformationRequirement, ReplaceInformationRequirement};
use crate::artifacts::program::diff::{ProgramInformationDelta, ProgramInformationPatchEntry};
use crate::artifacts::program::registers::InformationRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.information` on apply.
pub fn diff_create(payload: &CreateInformationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { information: Some(ProgramInformationDelta { added: vec![payload.information_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteInformationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { information: Some(ProgramInformationDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameInformationRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = InformationRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { information: Some(ProgramInformationDelta { patched: vec![ProgramInformationPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceInformationRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.information.iter().find(|row| row.header.id == payload.information_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.information_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { information: Some(ProgramInformationDelta { patched: vec![ProgramInformationPatchEntry { id: payload.information_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
