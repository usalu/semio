//! ↩️ Inverse (undo) construction for the `replace-organizational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏢organizational` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceOrganizationalRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.organizational.iter().find(|row| row.header.id == payload.organizational_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceOrganizationalRequirement(super::ReplaceOrganizationalRequirement { organizational_requirement: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
