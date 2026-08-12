//! 🔺️ Sparse diff construction for the `create-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::mutation::CreateIssue;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramIssuesDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.issues` on apply.
pub fn diff(payload: &CreateIssue, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { issues: Some(ProgramIssuesDelta { added: vec![payload.issue.clone()], ..Default::default() }), ..Default::default() }
}
