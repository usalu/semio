//! 🔺️ Sparse diff builder for `RemoveTag` — no-op (empty diff) when BASE doesn't have the tag.
use crate::diff::VcsTagsDelta;
use crate::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
/// 🔺️ Error `target-missing` when BASE doesn't have the tag.
pub fn diff(payload: &super::RemoveTag, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
    let Some(index) = base.tags.iter().position(|existing| existing == &payload.tag) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tag \"{}\" does not exist.", payload.tag), [payload.tag.clone()]);
    };
    protocol::MutationOutcome::new(VcsDiff { tags: Some(VcsTagsDelta::removal(&base.tags, index)), ..Default::default() })
}
//#endregion 🔖️Diff
