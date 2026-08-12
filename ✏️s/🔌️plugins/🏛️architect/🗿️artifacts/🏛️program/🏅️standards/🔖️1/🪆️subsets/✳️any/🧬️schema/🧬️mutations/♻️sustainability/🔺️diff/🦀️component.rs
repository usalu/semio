//! 🔺️ Sparse diff construction for the `sustainability` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateSustainabilityRequirement, DeleteSustainabilityRequirement, RenameSustainabilityRequirement, ReplaceSustainabilityRequirement};
use crate::artifacts::program::diff::{ProgramSustainabilityDelta, ProgramSustainabilityPatchEntry};
use crate::artifacts::program::registers::SustainabilityRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.sustainability` on apply.
pub fn diff_create(payload: &CreateSustainabilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { added: vec![payload.sustainability_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteSustainabilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameSustainabilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SustainabilityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { patched: vec![ProgramSustainabilityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceSustainabilityRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.sustainability.iter().find(|row| row.header.id == payload.sustainability_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.sustainability_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { patched: vec![ProgramSustainabilityPatchEntry { id: payload.sustainability_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
