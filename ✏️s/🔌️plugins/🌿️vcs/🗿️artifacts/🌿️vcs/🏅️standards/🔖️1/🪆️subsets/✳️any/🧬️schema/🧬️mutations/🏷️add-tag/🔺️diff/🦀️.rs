//! 🔺️ Sparse diff builder for `AddTag` — appends (or inserts at `index`), no-op (empty diff) when BASE already has the tag.
use crate::diff::VcsTagsDelta;
use crate::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
/// 🔺️ Warning `no-op` when BASE already has the tag.
pub fn diff(payload: &super::AddTag, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
    if base.tags.iter().any(|existing| existing == &payload.tag) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tag \"{}\" is already present.", payload.tag));
    }
    let reordered = payload.index.filter(|index| (*index as usize) < base.tags.len()).map(|index| {
        let mut order = base.tags.clone();
        order.insert(index as usize, payload.tag.clone());
        order
    });
    protocol::MutationOutcome::new(VcsDiff { tags: Some(VcsTagsDelta { added: vec![payload.tag.clone()], reordered, ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
