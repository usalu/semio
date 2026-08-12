//! ↩️ Inverse (undo) construction for the `create-operational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📋operations` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateOperationalRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteOperationalRequirement(super::super::delete_operational_requirement::mutation::DeleteOperationalRequirement { id: payload.operational_requirement.header.id.clone() })]
}
