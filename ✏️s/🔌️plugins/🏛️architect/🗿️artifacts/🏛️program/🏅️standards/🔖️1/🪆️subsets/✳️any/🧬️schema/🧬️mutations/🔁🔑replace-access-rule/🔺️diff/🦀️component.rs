//! 🔺️ Sparse diff construction for the `replace-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::mutation::ReplaceAccessRule;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAccessRulesDelta, ProgramAccessRulesPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceAccessRule, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.access_rules.iter().find(|row| row.header.id == payload.access_rule.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.access_rule).expect("diff_patch always produces a full patch");
    ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { patched: vec![ProgramAccessRulesPatchEntry { id: payload.access_rule.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
