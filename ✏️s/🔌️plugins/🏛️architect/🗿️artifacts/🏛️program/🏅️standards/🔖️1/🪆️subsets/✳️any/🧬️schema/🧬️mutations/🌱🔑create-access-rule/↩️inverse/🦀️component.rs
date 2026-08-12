//! ↩️ Inverse (undo) construction for the `create-access-rule` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔑access-rules` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateAccessRule, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAccessRule(super::super::delete_access_rule::mutation::DeleteAccessRule { id: payload.access_rule.header.id.clone() })]
}
