//! ↩️ Inverse (undo) construction for the `replace-security-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛡️security` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceSecurityRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.security.iter().find(|row| row.header.id == payload.security_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSecurityRequirement(super::ReplaceSecurityRequirement { security_requirement: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
