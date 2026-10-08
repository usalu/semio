//! ↩️ Inverse (undo) construction for the `delete-collaboration-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🤝collaboration` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteCollaborationRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.collaboration.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateCollaborationRecord(super::super::create_collaboration_record::CreateCollaborationRecord { collaboration_record: base.collaboration[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
