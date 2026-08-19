//! ↩️ Inverse (undo) construction for the `replace-document` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📄documents` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceDocument, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.artifacts.iter().find(|row| row.header.id == payload.document.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceDocument(super::mutation::ReplaceDocument { document: existing.clone() })],
        None => Vec::new(),
    }
}
