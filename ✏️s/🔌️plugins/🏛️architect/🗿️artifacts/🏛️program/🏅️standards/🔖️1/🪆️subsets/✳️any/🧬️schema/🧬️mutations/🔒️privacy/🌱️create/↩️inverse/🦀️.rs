//! ↩️ Inverse (undo) construction for the `create-privacy-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔒privacy` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreatePrivacyRequirement, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeletePrivacyRequirement(super::super::delete_privacy_requirement::DeletePrivacyRequirement { id: payload.privacy_requirement.header.id.clone() })]

    })())
}
