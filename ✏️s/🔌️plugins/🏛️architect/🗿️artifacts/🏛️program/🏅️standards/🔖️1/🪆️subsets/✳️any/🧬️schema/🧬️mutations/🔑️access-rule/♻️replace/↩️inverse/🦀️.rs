//! ↩️ Inverse (undo) construction for the `replace-access-rule` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔑access-rules` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceAccessRule, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.access_rules.iter().find(|row| row.header.id == payload.access_rule.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAccessRule(super::ReplaceAccessRule { access_rule: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
