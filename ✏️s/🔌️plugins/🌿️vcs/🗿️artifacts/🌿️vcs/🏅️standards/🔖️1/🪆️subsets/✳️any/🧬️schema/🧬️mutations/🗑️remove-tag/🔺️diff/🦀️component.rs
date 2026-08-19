//! 🔺️ Sparse diff builder for `RemoveTag` — no-op (empty diff) when BASE doesn't have the tag.
use crate::artifacts::vcs::diff::VcsTagsDelta;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
/// 🔺️ Error `target-missing` when BASE doesn't have the tag.
pub async fn diff(payload: &super::mutation::RemoveTag, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
    if !base.tags.iter().any(|existing| existing == &payload.tag) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tag \"{}\" does not exist.", payload.tag), [payload.tag.clone()]);
    }
    protocol::MutationOutcome::new(VcsDiff { tags: Some(VcsTagsDelta { removed: vec![payload.tag.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
