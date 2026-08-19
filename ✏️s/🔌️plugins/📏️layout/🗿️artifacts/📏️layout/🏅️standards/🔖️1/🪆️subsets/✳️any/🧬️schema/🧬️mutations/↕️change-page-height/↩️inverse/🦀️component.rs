//! ↩ Inverse constructor for `change-page-height` — reconstructed from captured BASE state.

use super::mutation::ChangePageHeight;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region ↕️ChangePageHeight
pub async fn inverse_change_page_height(payload: &ChangePageHeight, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::ChangePageHeight(ChangePageHeight { id: payload.id.clone(), new_height: page.height })],
        None => Vec::new(),
    }
}
//#endregion ↕️ChangePageHeight
