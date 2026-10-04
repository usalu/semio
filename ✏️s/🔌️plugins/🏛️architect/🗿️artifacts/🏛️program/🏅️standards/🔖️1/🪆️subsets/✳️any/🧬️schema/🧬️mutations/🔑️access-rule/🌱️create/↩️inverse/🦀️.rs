//! ↩️ Inverse (undo) construction for the `create-access-rule` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔑access-rules` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateAccessRule, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteAccessRule(super::super::delete_access_rule::DeleteAccessRule { id: payload.access_rule.header.id.clone() })]

    })())
}
