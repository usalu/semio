//! 🔺️ Sparse diff construction for the `delete-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::mutation::DeleteIssue;
use crate::artifacts::program::diff::ProgramIssuesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteIssue, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { issues: Some(ProgramIssuesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
