//! 🔺️ Sparse diff construction for the `security` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateSecurityRequirement, DeleteSecurityRequirement, RenameSecurityRequirement, ReplaceSecurityRequirement};
use crate::artifacts::program::diff::{ProgramSecurityDelta, ProgramSecurityPatchEntry};
use crate::artifacts::program::registers::SecurityRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.security` on apply.
pub fn diff_create(payload: &CreateSecurityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { security: Some(ProgramSecurityDelta { added: vec![payload.security_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteSecurityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { security: Some(ProgramSecurityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameSecurityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SecurityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { security: Some(ProgramSecurityDelta { patched: vec![ProgramSecurityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceSecurityRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.security.iter().find(|row| row.header.id == payload.security_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.security_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { security: Some(ProgramSecurityDelta { patched: vec![ProgramSecurityPatchEntry { id: payload.security_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
