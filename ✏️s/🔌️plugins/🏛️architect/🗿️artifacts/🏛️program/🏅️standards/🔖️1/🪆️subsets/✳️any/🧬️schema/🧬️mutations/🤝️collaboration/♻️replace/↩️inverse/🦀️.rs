//! ↩️ Inverse (undo) construction for the `replace-collaboration-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🤝collaboration` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceCollaborationRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.collaboration.iter().find(|row| row.header.id == payload.collaboration_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceCollaborationRecord(super::ReplaceCollaborationRecord { collaboration_record: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
