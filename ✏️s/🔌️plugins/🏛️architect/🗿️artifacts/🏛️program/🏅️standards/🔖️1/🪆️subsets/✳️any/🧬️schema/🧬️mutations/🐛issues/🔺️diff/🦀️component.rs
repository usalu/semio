//! 🔺️ Sparse diff construction for the `issues` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateIssue, DeleteIssue, RenameIssue, ReplaceIssue};
use crate::artifacts::program::diff::{ProgramIssuesDelta, ProgramIssuesPatchEntry};
use crate::artifacts::program::registers::IssuePatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.issues` on apply.
pub fn diff_create(payload: &CreateIssue, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { issues: Some(ProgramIssuesDelta { added: vec![payload.issue.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteIssue, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { issues: Some(ProgramIssuesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameIssue, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = IssuePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { issues: Some(ProgramIssuesDelta { patched: vec![ProgramIssuesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceIssue, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.issues.iter().find(|row| row.header.id == payload.issue.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.issue).expect("diff_patch always produces a full patch");
    ProgramDiff { issues: Some(ProgramIssuesDelta { patched: vec![ProgramIssuesPatchEntry { id: payload.issue.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
