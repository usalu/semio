//! ↩️ Inverse (undo) construction for the `delete-privacy-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔒privacy` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeletePrivacyRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.privacy.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreatePrivacyRequirement(super::super::create_privacy_requirement::CreatePrivacyRequirement { privacy_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
