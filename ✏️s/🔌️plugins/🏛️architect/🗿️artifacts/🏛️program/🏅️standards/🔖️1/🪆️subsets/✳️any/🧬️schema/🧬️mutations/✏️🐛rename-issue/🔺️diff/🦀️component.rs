//! 🔺️ Sparse diff construction for the `rename-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::mutation::RenameIssue;
use crate::artifacts::program::diff::{ProgramIssuesDelta, ProgramIssuesPatchEntry};
use crate::artifacts::program::registers::IssuePatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameIssue, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = IssuePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { issues: Some(ProgramIssuesDelta { patched: vec![ProgramIssuesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
