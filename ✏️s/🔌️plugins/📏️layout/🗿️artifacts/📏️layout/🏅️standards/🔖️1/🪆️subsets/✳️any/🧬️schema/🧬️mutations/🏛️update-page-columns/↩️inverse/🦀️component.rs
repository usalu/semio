//! ↩ Inverse constructor for `update-page-columns` — reconstructed from captured BASE state.

use super::mutation::UpdatePageColumns;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 🏛️UpdatePageColumns
pub async fn inverse_update_page_columns(payload: &UpdatePageColumns, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::UpdatePageColumns(UpdatePageColumns { id: payload.id.clone(), count: page.columns.count, gutter: page.columns.gutter })],
        None => Vec::new(),
    }
}
//#endregion 🏛️UpdatePageColumns
