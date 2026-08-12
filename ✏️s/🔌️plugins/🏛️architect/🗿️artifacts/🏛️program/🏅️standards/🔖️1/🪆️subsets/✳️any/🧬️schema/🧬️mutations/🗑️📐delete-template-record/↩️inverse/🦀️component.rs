//! ↩️ Inverse (undo) construction for the `delete-template-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📐templates` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteTemplateRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.templates.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateTemplateRecord(super::super::create_template_record::mutation::CreateTemplateRecord { template_record: existing.clone() })],
        None => Vec::new(),
    }
}
