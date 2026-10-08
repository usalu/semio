//! ↩️ Inverse (undo) construction for the `delete-organizational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏢organizational` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteOrganizationalRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.organizational.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateOrganizationalRequirement(super::super::create_organizational_requirement::CreateOrganizationalRequirement { organizational_requirement: base.organizational[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
