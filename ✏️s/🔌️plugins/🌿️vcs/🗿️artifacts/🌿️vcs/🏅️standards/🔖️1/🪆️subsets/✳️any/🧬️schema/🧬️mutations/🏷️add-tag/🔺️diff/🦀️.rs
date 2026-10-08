//! 🔺️ Sparse diff builder for `AddTag` — appends (or inserts at `index`), no-op (empty diff) when BASE already has the tag.
use crate::diff::VcsTagsDelta;
use crate::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
/// 🔺️ Warning `no-op` when BASE already has the tag.
pub fn diff(payload: &super::AddTag, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
    if base.tags.iter().any(|existing| existing == &payload.tag) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tag \"{}\" is already present.", payload.tag));
    }
    let at = payload.index.map_or(base.tags.len(), |index| (index as usize).min(base.tags.len()));
    protocol::MutationOutcome::new(VcsDiff { tags: Some(VcsTagsDelta::insertion(at, payload.tag.clone())), ..Default::default() })
}
//#endregion 🔖️Diff
