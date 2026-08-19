//! ↩️ Inverse (undo) construction for the `create-accessibility-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `♿accessibility` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateAccessibilityRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAccessibilityRequirement(super::super::delete_accessibility_requirement::mutation::DeleteAccessibilityRequirement { id: payload.accessibility_requirement.header.id.clone() })]
}
