//! ↩ Inverse constructor for `change-page-width` — reconstructed from captured BASE state.

use super::mutation::ChangePageWidth;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region ↔️ChangePageWidth
pub async fn inverse_change_page_width(payload: &ChangePageWidth, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::ChangePageWidth(ChangePageWidth { id: payload.id.clone(), new_width: page.width })],
        None => Vec::new(),
    }
}
//#endregion ↔️ChangePageWidth
