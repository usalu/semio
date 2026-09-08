//! ↩️ Inverse (undo) construction for the `replace-document` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📄documents` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceDocument, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.artifacts.iter().find(|row| row.header.id == payload.document.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceDocument(super::ReplaceDocument { document: existing.clone() })],
        None => Vec::new(),
    }
}
