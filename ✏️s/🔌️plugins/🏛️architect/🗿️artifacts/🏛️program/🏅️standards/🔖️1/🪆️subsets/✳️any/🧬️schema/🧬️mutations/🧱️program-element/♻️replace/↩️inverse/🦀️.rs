//! ↩️ Inverse (undo) construction for the `replace-program-element` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧱elements` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceProgramElement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.elements.iter().find(|row| row.header.id == payload.program_element.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceProgramElement(super::ReplaceProgramElement { program_element: existing.clone() })],
        None => Vec::new(),
    }
}
