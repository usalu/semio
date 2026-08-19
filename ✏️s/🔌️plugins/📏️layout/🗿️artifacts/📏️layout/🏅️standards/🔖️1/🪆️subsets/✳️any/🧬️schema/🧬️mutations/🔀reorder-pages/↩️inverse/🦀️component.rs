//! ↩ Inverse constructor for `reorder-pages` — reorders back to the captured BASE-state position.

use super::mutation::ReorderPages;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 🔀ReorderPages
pub async fn inverse_reorder_pages(payload: &ReorderPages, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().position(|page| page.id == payload.id) {
        Some(original_index) => vec![LayoutMutation::ReorderPages(ReorderPages { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }
}
//#endregion 🔀ReorderPages
