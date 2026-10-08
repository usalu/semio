//! ↩️ Inverse (undo) construction for the `replace-document` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📄documents` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by deleting the replacement row and recreating the pre-state row at its original index (rows replay last-to-first, so the create is listed first). Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceDocument, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.artifacts.iter().position(|row| row.header.id == payload.document.header.id) {
        Some(position) => vec![
            ProgramMutation::CreateDocument(super::super::create_document::CreateDocument { document: base.artifacts[position].clone(), index: Some(position) }),
            ProgramMutation::DeleteDocument(super::super::delete_document::DeleteDocument { id: payload.document.header.id.clone() }),
        ],
        None => Vec::new(),
    }

    })())
}
