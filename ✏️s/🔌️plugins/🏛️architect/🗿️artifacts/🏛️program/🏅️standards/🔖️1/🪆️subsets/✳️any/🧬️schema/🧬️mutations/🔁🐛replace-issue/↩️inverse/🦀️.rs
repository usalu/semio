//! ↩️ Inverse (undo) construction for the `replace-issue` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🐛issues` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceIssue, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.issues.iter().find(|row| row.header.id == payload.issue.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceIssue(super::ReplaceIssue { issue: existing.clone() })],
        None => Vec::new(),
    }
}
