//! ↩️ Inverse (undo) construction for the `create-template-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📐templates` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateTemplateRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteTemplateRecord(super::super::delete_template_record::mutation::DeleteTemplateRecord { id: payload.template_record.header.id.clone() })]
}
