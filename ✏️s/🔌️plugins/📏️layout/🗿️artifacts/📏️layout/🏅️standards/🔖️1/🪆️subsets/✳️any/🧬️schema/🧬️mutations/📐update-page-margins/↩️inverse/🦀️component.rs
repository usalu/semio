//! ↩ Inverse constructor for `update-page-margins` — reconstructed from captured BASE state.

use super::mutation::UpdatePageMargins;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 📐UpdatePageMargins
pub async fn inverse_update_page_margins(payload: &UpdatePageMargins, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::UpdatePageMargins(UpdatePageMargins { id: payload.id.clone(), top: page.margins.top, right: page.margins.right, bottom: page.margins.bottom, left: page.margins.left })],
        None => Vec::new(),
    }
}
//#endregion 📐UpdatePageMargins
