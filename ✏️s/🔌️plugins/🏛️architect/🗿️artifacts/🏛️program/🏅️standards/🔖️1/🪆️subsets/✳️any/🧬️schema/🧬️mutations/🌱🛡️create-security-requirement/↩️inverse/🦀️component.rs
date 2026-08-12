//! ↩️ Inverse (undo) construction for the `create-security-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛡️security` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateSecurityRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSecurityRequirement(super::super::delete_security_requirement::mutation::DeleteSecurityRequirement { id: payload.security_requirement.header.id.clone() })]
}
