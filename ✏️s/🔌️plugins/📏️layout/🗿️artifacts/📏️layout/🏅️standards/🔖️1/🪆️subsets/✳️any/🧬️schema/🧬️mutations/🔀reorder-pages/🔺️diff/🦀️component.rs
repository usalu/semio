//! 🔺 Diff constructor for `reorder-pages`.

use super::mutation::ReorderPages;
use crate::artifacts::layout::schema::diff::LayoutPagesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🔀ReorderPages
pub fn diff_reorder_pages(payload: &ReorderPages, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.pages.iter().any(|page| page.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let current: Vec<String> = base.pages.iter().map(|page| page.id.clone()).collect();
    let mut ids = current.clone();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    if ids == current {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" is already at the requested position.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() })
}
//#endregion 🔀ReorderPages
