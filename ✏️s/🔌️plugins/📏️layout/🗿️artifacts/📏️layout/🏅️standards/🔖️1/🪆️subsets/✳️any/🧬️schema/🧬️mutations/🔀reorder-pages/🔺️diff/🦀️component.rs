//! 🔺 Diff constructor for `reorder-pages`.

use super::mutation::ReorderPages;
use crate::artifacts::layout::schema::diff::LayoutPagesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🔀ReorderPages
pub fn diff_reorder_pages(payload: &ReorderPages, base: &LayoutSnapshot) -> LayoutDiff {
    let mut ids: Vec<String> = base.pages.iter().map(|page| page.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    LayoutDiff { pages: Some(LayoutPagesDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
//#endregion 🔀ReorderPages
