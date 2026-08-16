//! 🔺️ Sparse diff builder for `AddTag` — no-op (empty diff) when BASE already has the tag.
use crate::artifacts::vcs::diff::VcsTagsDelta;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
/// 🔺️ Warning `no-op` when BASE already has the tag.
pub fn diff(payload: &super::mutation::AddTag, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
    if base.tags.iter().any(|existing| existing == &payload.tag) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tag \"{}\" is already present.", payload.tag));
    }
    protocol::MutationOutcome::new(VcsDiff { tags: Some(VcsTagsDelta { added: vec![payload.tag.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
