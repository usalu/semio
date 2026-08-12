//! 🔺️ Sparse diff construction for the `access_rules` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateAccessRule, DeleteAccessRule, RenameAccessRule, ReplaceAccessRule};
use crate::artifacts::program::diff::{ProgramAccessRulesDelta, ProgramAccessRulesPatchEntry};
use crate::artifacts::program::registers::AccessRulePatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.access_rules` on apply.
pub fn diff_create(payload: &CreateAccessRule, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { added: vec![payload.access_rule.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteAccessRule, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameAccessRule, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AccessRulePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { patched: vec![ProgramAccessRulesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceAccessRule, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.access_rules.iter().find(|row| row.header.id == payload.access_rule.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.access_rule).expect("diff_patch always produces a full patch");
    ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { patched: vec![ProgramAccessRulesPatchEntry { id: payload.access_rule.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
