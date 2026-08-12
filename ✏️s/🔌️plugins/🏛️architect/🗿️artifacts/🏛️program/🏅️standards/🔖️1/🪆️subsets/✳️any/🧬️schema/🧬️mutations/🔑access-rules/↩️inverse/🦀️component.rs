//! ↩️ Inverse (undo) construction for the `access_rules` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateAccessRule, DeleteAccessRule, RenameAccessRule, ReplaceAccessRule};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateAccessRule, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAccessRule(DeleteAccessRule { id: payload.access_rule.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteAccessRule, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.access_rules.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateAccessRule(CreateAccessRule { access_rule: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameAccessRule, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.access_rules.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameAccessRule(RenameAccessRule { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceAccessRule, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.access_rules.iter().find(|row| row.header.id == payload.access_rule.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAccessRule(ReplaceAccessRule { access_rule: existing.clone() })],
        None => Vec::new(),
    }
}
