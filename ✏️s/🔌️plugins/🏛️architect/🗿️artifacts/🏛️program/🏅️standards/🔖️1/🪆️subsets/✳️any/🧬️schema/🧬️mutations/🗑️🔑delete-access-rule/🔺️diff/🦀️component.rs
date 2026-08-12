//! 🔺️ Sparse diff construction for the `delete-access-rule` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔑access-rules` per Wave C.

use super::mutation::DeleteAccessRule;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAccessRulesDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteAccessRule, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { access_rules: Some(ProgramAccessRulesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
