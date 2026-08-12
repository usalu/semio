//! 🔺️ Sparse diff construction for the `services` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateServiceRequirement, DeleteServiceRequirement, RenameServiceRequirement, ReplaceServiceRequirement};
use crate::artifacts::program::diff::{ProgramServicesDelta, ProgramServicesPatchEntry};
use crate::artifacts::program::registers::ServiceRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.services` on apply.
pub fn diff_create(payload: &CreateServiceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { services: Some(ProgramServicesDelta { added: vec![payload.service_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteServiceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { services: Some(ProgramServicesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameServiceRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ServiceRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { services: Some(ProgramServicesDelta { patched: vec![ProgramServicesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceServiceRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.services.iter().find(|row| row.header.id == payload.service_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.service_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { services: Some(ProgramServicesDelta { patched: vec![ProgramServicesPatchEntry { id: payload.service_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
