//! 🔺️ `replace-part-number-rule` — sparse diff construction.

use super::mutation::ReplacePartNumberRule;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplacePartNumberRule, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.part_number_rule == payload.new_rule {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Part number rule already has this value.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { part_number_rule: Some(payload.new_rule.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
