//! ↩️ Inverse (undo) construction for the `delete-issue` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🐛issues` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteIssue, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.issues.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateIssue(super::super::create_issue::CreateIssue { issue: base.issues[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
