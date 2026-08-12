//! 🔺️ Sparse diff construction for the `rename-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::mutation::RenameAccessRule;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAccessRulesDelta, ProgramAccessRulesPatchEntry};
use crate::artifacts::program::registers::AccessRulePatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameAccessRule, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AccessRulePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { patched: vec![ProgramAccessRulesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
