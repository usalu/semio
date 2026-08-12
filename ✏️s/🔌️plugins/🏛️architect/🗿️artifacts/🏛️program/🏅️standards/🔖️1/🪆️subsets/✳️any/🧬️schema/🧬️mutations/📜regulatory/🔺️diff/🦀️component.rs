//! 🔺️ Sparse diff construction for the `regulatory` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateRegulatoryRequirement, DeleteRegulatoryRequirement, RenameRegulatoryRequirement, ReplaceRegulatoryRequirement};
use crate::artifacts::program::diff::{ProgramRegulatoryDelta, ProgramRegulatoryPatchEntry};
use crate::artifacts::program::registers::RegulatoryRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.regulatory` on apply.
pub fn diff_create(payload: &CreateRegulatoryRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { added: vec![payload.regulatory_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteRegulatoryRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameRegulatoryRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = RegulatoryRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { patched: vec![ProgramRegulatoryPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceRegulatoryRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.regulatory.iter().find(|row| row.header.id == payload.regulatory_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.regulatory_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { patched: vec![ProgramRegulatoryPatchEntry { id: payload.regulatory_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
