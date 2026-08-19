//! ↩️ Inverse (undo) construction for the `delete-security-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛡️security` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteSecurityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.security.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSecurityRequirement(super::super::create_security_requirement::mutation::CreateSecurityRequirement { security_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
