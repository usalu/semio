//! 🔺️ Sparse diff builder for `RemoveTag` — no-op (empty diff) when BASE doesn't have the tag.
use crate::artifacts::vcs::diff::VcsTagsDelta;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveTag, base: &VcsSnapshot) -> VcsDiff {
    if base.tags.iter().any(|existing| existing == &payload.tag) {
        VcsDiff { tags: Some(VcsTagsDelta { removed: vec![payload.tag.clone()], ..Default::default() }), ..Default::default() }
    } else {
        VcsDiff::default()
    }
}
//#endregion 🔖️Diff
