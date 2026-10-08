//! 🔺️ Sparse diff construction for the `create-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::CreateIssue;
use crate::diff::ProgramIssuesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateIssue, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.issue.header.id;
    if base.issues.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An issue already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.issues.len());
    if at > base.issues.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the issue list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { issues: Some(ProgramIssuesDelta::insertion(at, payload.issue.clone())), ..Default::default() })
}
