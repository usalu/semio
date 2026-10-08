//! ↩️ Inverse (undo) construction for the `delete-template-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📐templates` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteTemplateRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.templates.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateTemplateRecord(super::super::create_template_record::CreateTemplateRecord { template_record: base.templates[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
