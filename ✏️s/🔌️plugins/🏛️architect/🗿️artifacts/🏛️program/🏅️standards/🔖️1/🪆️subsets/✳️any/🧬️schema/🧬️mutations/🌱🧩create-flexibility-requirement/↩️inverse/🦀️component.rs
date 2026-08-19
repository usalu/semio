//! ↩️ Inverse (undo) construction for the `create-flexibility-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧩flexibility` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateFlexibilityRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteFlexibilityRequirement(super::super::delete_flexibility_requirement::mutation::DeleteFlexibilityRequirement { id: payload.flexibility_requirement.header.id.clone() })]
}
