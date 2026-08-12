//! ↩️ Inverse (undo) construction for the `delete-document` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📄documents` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteDocument, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.artifacts.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateDocument(super::super::create_document::mutation::CreateDocument { document: existing.clone() })],
        None => Vec::new(),
    }
}
