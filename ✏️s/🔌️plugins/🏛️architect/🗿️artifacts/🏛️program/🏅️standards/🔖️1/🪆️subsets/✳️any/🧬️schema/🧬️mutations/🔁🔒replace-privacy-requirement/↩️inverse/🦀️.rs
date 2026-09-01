//! ↩️ Inverse (undo) construction for the `replace-privacy-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔒privacy` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplacePrivacyRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.privacy.iter().find(|row| row.header.id == payload.privacy_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplacePrivacyRequirement(super::ReplacePrivacyRequirement { privacy_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
