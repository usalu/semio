//! 🔺️ Sparse diff construction for the `replace-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::ReplaceIssue;
use crate::diff::ProgramIssuesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceIssue, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.issue.header.id;
    let Some(position) = base.issues.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No issue exists with this id.", [id.0.clone()]);
    };
    if base.issues[position] == payload.issue {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This issue already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramIssuesDelta::removal(&base.issues, position);
    delta.absorb(ProgramIssuesDelta::insertion(position, payload.issue.clone()));
    protocol::MutationOutcome::new(ProgramDiff { issues: Some(delta), ..Default::default() })
}
