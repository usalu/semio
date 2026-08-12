//! ↩️ Inverse (undo) construction for the `replace-access-rule` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔑access-rules` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceAccessRule, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.access_rules.iter().find(|row| row.header.id == payload.access_rule.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAccessRule(super::mutation::ReplaceAccessRule { access_rule: existing.clone() })],
        None => Vec::new(),
    }
}
