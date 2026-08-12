//! 🔺️ Sparse diff construction for the `privacy` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreatePrivacyRequirement, DeletePrivacyRequirement, RenamePrivacyRequirement, ReplacePrivacyRequirement};
use crate::artifacts::program::diff::{ProgramPrivacyDelta, ProgramPrivacyPatchEntry};
use crate::artifacts::program::registers::PrivacyRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.privacy` on apply.
pub fn diff_create(payload: &CreatePrivacyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { privacy: Some(ProgramPrivacyDelta { added: vec![payload.privacy_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeletePrivacyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { privacy: Some(ProgramPrivacyDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenamePrivacyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = PrivacyRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { privacy: Some(ProgramPrivacyDelta { patched: vec![ProgramPrivacyPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplacePrivacyRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.privacy.iter().find(|row| row.header.id == payload.privacy_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.privacy_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { privacy: Some(ProgramPrivacyDelta { patched: vec![ProgramPrivacyPatchEntry { id: payload.privacy_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
