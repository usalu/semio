//! ↩️ Inverse (undo) construction for the `create-information-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `ℹ️information` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::CreateInformationRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteInformationRequirement(super::super::delete_information_requirement::DeleteInformationRequirement { id: payload.information_requirement.header.id.clone() })]
}
