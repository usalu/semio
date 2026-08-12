//! ↩️ Inverse (undo) construction for the `create-wayfinding-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧭wayfinding` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateWayfindingRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteWayfindingRequirement(super::super::delete_wayfinding_requirement::mutation::DeleteWayfindingRequirement { id: payload.wayfinding_requirement.header.id.clone() })]
}
