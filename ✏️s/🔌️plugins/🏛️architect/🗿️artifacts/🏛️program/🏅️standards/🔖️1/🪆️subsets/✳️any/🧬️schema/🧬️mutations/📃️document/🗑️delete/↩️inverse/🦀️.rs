//! ↩️ Inverse (undo) construction for the `delete-document` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📄documents` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteDocument, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.artifacts.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateDocument(super::super::create_document::CreateDocument { document: base.artifacts[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
