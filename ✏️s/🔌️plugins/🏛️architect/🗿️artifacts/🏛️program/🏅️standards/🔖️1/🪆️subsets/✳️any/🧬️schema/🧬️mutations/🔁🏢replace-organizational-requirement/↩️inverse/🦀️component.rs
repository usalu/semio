//! ↩️ Inverse (undo) construction for the `replace-organizational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏢organizational` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceOrganizationalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.organizational.iter().find(|row| row.header.id == payload.organizational_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceOrganizationalRequirement(super::mutation::ReplaceOrganizationalRequirement { organizational_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
