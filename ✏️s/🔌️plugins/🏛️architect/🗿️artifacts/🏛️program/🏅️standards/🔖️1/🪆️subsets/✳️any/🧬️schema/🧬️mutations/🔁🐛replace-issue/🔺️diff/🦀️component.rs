//! 🔺️ Sparse diff construction for the `replace-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::mutation::ReplaceIssue;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramIssuesDelta, ProgramIssuesPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceIssue, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.issues.iter().find(|row| row.header.id == payload.issue.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.issue).expect("diff_patch always produces a full patch");
    ProgramDiff { issues: Some(ProgramIssuesDelta { patched: vec![ProgramIssuesPatchEntry { id: payload.issue.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
