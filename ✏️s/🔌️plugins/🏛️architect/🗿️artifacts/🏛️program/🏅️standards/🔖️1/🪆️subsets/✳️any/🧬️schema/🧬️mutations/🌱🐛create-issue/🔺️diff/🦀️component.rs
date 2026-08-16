//! 🔺️ Sparse diff construction for the `create-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::mutation::CreateIssue;
use crate::artifacts::program::diff::ProgramIssuesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateIssue, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.issue.header.id.clone();
    if base.issues.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An issue already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { issues: Some(ProgramIssuesDelta { added: vec![payload.issue.clone()], ..Default::default() }), ..Default::default() })
}
