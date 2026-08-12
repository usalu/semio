//! ↩️ Inverse (undo) construction for the `create-privacy-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔒privacy` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreatePrivacyRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeletePrivacyRequirement(super::super::delete_privacy_requirement::mutation::DeletePrivacyRequirement { id: payload.privacy_requirement.header.id.clone() })]
}
