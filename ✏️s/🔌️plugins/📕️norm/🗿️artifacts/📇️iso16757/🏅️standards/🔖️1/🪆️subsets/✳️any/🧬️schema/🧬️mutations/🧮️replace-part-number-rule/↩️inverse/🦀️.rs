//! ↩️ `replace-part-number-rule` — undo restores BASE's whole rule.

use crate::replace_part_number_rule::mutation::ReplacePartNumberRule;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ReplacePartNumberRule, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::replace_part_number_rule::mutation::ReplacePartNumberRule> {
    Ok((|| {
    vec![Op::from(ReplacePartNumberRule { new_rule: base.part_number_rule.clone() })]

    })())
}
//#endregion 🔖️Inverse
