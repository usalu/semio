//! ↩️ `replace-part-number-rule` — undo restores BASE's whole rule.

use super::mutation::ReplacePartNumberRule;
use crate::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ReplacePartNumberRule, base: &Iso16757Snapshot) -> Result<Vec<Iso16757Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Iso16757Mutation::ReplacePartNumberRule(ReplacePartNumberRule { new_rule: base.part_number_rule.clone() })]

    })())
}
//#endregion 🔖️Inverse
