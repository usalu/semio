//! 🔺️ `replace-part-number-rule` — sparse diff construction.

use super::mutation::ReplacePartNumberRule;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplacePartNumberRule, _base: &Iso16757Snapshot) -> Iso16757Diff {
    Iso16757Diff { part_number_rule: Some(payload.new_rule.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
