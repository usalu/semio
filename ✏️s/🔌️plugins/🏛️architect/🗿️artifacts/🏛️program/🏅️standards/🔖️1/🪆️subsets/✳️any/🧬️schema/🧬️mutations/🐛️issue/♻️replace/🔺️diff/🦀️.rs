//! 🔺️ Sparse diff construction for the `replace-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::ReplaceIssue;
use crate::diff::ProgramIssuesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceIssue, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.issue.header.id;
    let Some(position) = base.issues.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No issue exists with this id.", [id.0.clone()]);
    };
    if base.issues[position] == payload.issue {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This issue already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.issues.len()).then(|| base.issues.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { issues: Some(ProgramIssuesDelta { removed: vec![id.0.clone()], added: vec![payload.issue.clone()], reordered, ..Default::default() }), ..Default::default() })
}
