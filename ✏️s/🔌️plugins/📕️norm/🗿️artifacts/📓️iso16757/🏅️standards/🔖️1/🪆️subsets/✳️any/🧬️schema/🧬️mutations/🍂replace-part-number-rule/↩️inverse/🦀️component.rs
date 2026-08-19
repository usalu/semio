//! ↩️ `replace-part-number-rule` — undo restores BASE's whole rule.

use super::mutation::ReplacePartNumberRule;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ReplacePartNumberRule, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    vec![Iso16757Mutation::ReplacePartNumberRule(ReplacePartNumberRule { new_rule: base.part_number_rule.clone() })]
}
//#endregion 🔖️Inverse
