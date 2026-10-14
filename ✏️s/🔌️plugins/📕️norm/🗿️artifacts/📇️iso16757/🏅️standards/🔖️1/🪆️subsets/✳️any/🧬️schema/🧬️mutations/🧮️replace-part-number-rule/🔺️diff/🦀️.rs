//! 🔺️ `replace-part-number-rule` — sparse diff construction.

use crate::replace_part_number_rule::mutation::ReplacePartNumberRule;
use semio_s_artifact_norm_iso16757_diff::Iso16757Diff;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplacePartNumberRule, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.part_number_rule == payload.new_rule {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Part number rule already has this value.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { part_number_rule: Some(payload.new_rule.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
