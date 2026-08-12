//! 🔺️ Sparse diff builder for `AddTag` — no-op (empty diff) when BASE already has the tag.
use crate::artifacts::vcs::diff::VcsTagsDelta;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddTag, base: &VcsSnapshot) -> VcsDiff {
    if base.tags.iter().any(|existing| existing == &payload.tag) {
        VcsDiff::default()
    } else {
        VcsDiff { tags: Some(VcsTagsDelta { added: vec![payload.tag.clone()], ..Default::default() }), ..Default::default() }
    }
}
//#endregion 🔖️Diff
