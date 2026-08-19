//! ↩️ Inverse (undo) construction for the `replace-accessibility-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `♿accessibility` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceAccessibilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.accessibility.iter().find(|row| row.header.id == payload.accessibility_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAccessibilityRequirement(super::mutation::ReplaceAccessibilityRequirement { accessibility_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
