//! ↩️ Inverse (undo) construction for the `create-sustainability-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `♻️sustainability` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateSustainabilityRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSustainabilityRequirement(super::super::delete_sustainability_requirement::mutation::DeleteSustainabilityRequirement { id: payload.sustainability_requirement.header.id.clone() })]
}
