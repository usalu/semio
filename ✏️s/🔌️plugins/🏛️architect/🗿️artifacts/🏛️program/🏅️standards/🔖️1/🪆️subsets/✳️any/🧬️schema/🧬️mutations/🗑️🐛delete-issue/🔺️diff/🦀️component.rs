//! 🔺️ Sparse diff construction for the `delete-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::mutation::DeleteIssue;
use crate::artifacts::program::diff::ProgramIssuesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteIssue, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.issues.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No issue exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { issues: Some(ProgramIssuesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
