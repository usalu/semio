//! ↩️ Inverse (undo) construction for the `create-organizational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏢organizational` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::CreateOrganizationalRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteOrganizationalRequirement(super::super::delete_organizational_requirement::DeleteOrganizationalRequirement { id: payload.organizational_requirement.header.id.clone() })]
}
