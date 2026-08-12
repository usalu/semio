//! 🔺️ Sparse diff construction for the `stakeholders` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateStakeholder, DeleteStakeholder, RenameStakeholder, ReplaceStakeholder};
use crate::artifacts::program::diff::{ProgramStakeholdersDelta, ProgramStakeholdersPatchEntry};
use crate::artifacts::program::registers::StakeholderPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.stakeholders` on apply.
pub fn diff_create(payload: &CreateStakeholder, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { added: vec![payload.stakeholder.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteStakeholder, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameStakeholder, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = StakeholderPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceStakeholder, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.stakeholders.iter().find(|row| row.header.id == payload.stakeholder.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.stakeholder).expect("diff_patch always produces a full patch");
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: payload.stakeholder.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
