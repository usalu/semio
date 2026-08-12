//! 🔺️ Sparse diff construction for the `accessibility` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateAccessibilityRequirement, DeleteAccessibilityRequirement, RenameAccessibilityRequirement, ReplaceAccessibilityRequirement};
use crate::artifacts::program::diff::{ProgramAccessibilityDelta, ProgramAccessibilityPatchEntry};
use crate::artifacts::program::registers::AccessibilityRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.accessibility` on apply.
pub fn diff_create(payload: &CreateAccessibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { added: vec![payload.accessibility_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteAccessibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameAccessibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AccessibilityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { patched: vec![ProgramAccessibilityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceAccessibilityRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.accessibility.iter().find(|row| row.header.id == payload.accessibility_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.accessibility_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { patched: vec![ProgramAccessibilityPatchEntry { id: payload.accessibility_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
