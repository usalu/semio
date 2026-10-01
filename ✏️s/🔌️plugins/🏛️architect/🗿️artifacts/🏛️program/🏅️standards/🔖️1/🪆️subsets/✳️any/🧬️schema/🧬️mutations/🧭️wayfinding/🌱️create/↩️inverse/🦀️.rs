//! ↩️ Inverse (undo) construction for the `create-wayfinding-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧭wayfinding` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateWayfindingRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteWayfindingRequirement(super::super::delete_wayfinding_requirement::DeleteWayfindingRequirement { id: payload.wayfinding_requirement.header.id.clone() })]
}
