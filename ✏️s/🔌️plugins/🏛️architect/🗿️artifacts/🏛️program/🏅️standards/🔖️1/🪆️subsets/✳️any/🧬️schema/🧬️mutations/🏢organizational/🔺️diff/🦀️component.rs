//! 🔺️ Sparse diff construction for the `organizational` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateOrganizationalRequirement, DeleteOrganizationalRequirement, RenameOrganizationalRequirement, ReplaceOrganizationalRequirement};
use crate::artifacts::program::diff::{ProgramOrganizationalDelta, ProgramOrganizationalPatchEntry};
use crate::artifacts::program::registers::OrganizationalRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.organizational` on apply.
pub fn diff_create(payload: &CreateOrganizationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { organizational: Some(ProgramOrganizationalDelta { added: vec![payload.organizational_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteOrganizationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { organizational: Some(ProgramOrganizationalDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameOrganizationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = OrganizationalRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { organizational: Some(ProgramOrganizationalDelta { patched: vec![ProgramOrganizationalPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceOrganizationalRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.organizational.iter().find(|row| row.header.id == payload.organizational_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.organizational_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { organizational: Some(ProgramOrganizationalDelta { patched: vec![ProgramOrganizationalPatchEntry { id: payload.organizational_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
