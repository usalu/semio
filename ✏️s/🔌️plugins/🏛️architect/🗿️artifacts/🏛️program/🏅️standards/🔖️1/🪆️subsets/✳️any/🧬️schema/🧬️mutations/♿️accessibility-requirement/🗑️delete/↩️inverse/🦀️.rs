//! ↩️ Inverse (undo) construction for the `delete-accessibility-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `♿accessibility` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteAccessibilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.accessibility.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateAccessibilityRequirement(super::super::create_accessibility_requirement::CreateAccessibilityRequirement { accessibility_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
