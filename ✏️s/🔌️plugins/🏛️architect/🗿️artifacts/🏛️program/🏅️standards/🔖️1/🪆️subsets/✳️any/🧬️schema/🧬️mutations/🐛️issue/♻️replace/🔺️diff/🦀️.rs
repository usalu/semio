//! 🔺️ Sparse diff construction for the `replace-issue` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🐛issues` per Wave C.

use super::ReplaceIssue;
use crate::artifacts::program::diff::{ProgramIssuesDelta, ProgramIssuesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceIssue, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.issues.iter().find(|row| row.header.id == payload.issue.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No issue exists with this id.", [payload.issue.header.id.0.clone()]);
    };
    if existing == &payload.issue {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This issue already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.issue).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { issues: Some(ProgramIssuesDelta { patched: vec![ProgramIssuesPatchEntry { id: payload.issue.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
