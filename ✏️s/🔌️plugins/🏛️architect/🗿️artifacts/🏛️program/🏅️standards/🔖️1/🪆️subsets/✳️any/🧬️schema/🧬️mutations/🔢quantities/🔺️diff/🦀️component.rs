//! 🔺️ Sparse diff construction for the `quantities` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateQuantityRequirement, DeleteQuantityRequirement, RenameQuantityRequirement, ReplaceQuantityRequirement};
use crate::artifacts::program::diff::{ProgramQuantitiesDelta, ProgramQuantitiesPatchEntry};
use crate::artifacts::program::registers::QuantityRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.quantities` on apply.
pub fn diff_create(payload: &CreateQuantityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { quantities: Some(ProgramQuantitiesDelta { added: vec![payload.quantity_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteQuantityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { quantities: Some(ProgramQuantitiesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameQuantityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = QuantityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { quantities: Some(ProgramQuantitiesDelta { patched: vec![ProgramQuantitiesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceQuantityRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.quantities.iter().find(|row| row.header.id == payload.quantity_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.quantity_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { quantities: Some(ProgramQuantitiesDelta { patched: vec![ProgramQuantitiesPatchEntry { id: payload.quantity_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
