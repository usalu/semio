//! ↩️ Inverse (undo) construction for the `create-issue` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🐛issues` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateIssue, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteIssue(super::super::delete_issue::mutation::DeleteIssue { id: payload.issue.header.id.clone() })]
}
