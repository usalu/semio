//! ↩️ Inverse (undo) construction for the `create-regulatory-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📜regulatory` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::CreateRegulatoryRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteRegulatoryRequirement(super::super::delete_regulatory_requirement::DeleteRegulatoryRequirement { id: payload.regulatory_requirement.header.id.clone() })]
}
