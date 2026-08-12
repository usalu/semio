//! ↩️ Inverse (undo) construction for the `delete-organizational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏢organizational` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteOrganizationalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.organizational.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateOrganizationalRequirement(super::super::create_organizational_requirement::mutation::CreateOrganizationalRequirement { organizational_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
