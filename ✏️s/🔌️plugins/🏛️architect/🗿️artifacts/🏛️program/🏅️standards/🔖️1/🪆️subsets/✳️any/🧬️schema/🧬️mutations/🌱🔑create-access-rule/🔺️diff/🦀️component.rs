//! 🔺️ Sparse diff construction for the `create-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::mutation::CreateAccessRule;
use crate::artifacts::program::diff::ProgramAccessRulesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.access_rules` on apply.
pub fn diff(payload: &CreateAccessRule, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { added: vec![payload.access_rule.clone()], ..Default::default() }), ..Default::default() }
}
