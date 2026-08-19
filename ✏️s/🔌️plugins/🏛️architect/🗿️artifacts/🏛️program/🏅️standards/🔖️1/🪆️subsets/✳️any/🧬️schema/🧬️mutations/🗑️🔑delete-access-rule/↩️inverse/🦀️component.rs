//! ↩️ Inverse (undo) construction for the `delete-access-rule` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔑access-rules` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteAccessRule, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.access_rules.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateAccessRule(super::super::create_access_rule::mutation::CreateAccessRule { access_rule: existing.clone() })],
        None => Vec::new(),
    }
}
