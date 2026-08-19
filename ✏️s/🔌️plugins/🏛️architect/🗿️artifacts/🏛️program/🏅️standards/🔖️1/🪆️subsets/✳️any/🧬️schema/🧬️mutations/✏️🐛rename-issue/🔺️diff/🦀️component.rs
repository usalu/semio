//! 🔺️ Sparse diff construction for the `rename-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::mutation::RenameIssue;
use crate::artifacts::program::diff::{ProgramIssuesDelta, ProgramIssuesPatchEntry};
use crate::artifacts::program::registers::IssuePatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameIssue, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.issues.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No issue exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This issue already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = IssuePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { issues: Some(ProgramIssuesDelta { patched: vec![ProgramIssuesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
