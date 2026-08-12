//! ↩ Inverse constructor for `delete-page` — captures the removed page's full payload and position.

use super::mutation::DeletePage;
use crate::artifacts::layout::mutations::{create_page, LayoutMutation};
use crate::artifacts::layout::LayoutSnapshot;

//#region 🗑️DeletePage
pub fn inverse_delete_page(payload: &DeletePage, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().position(|page| page.id == payload.id) {
        Some(index) => vec![LayoutMutation::CreatePage(create_page::mutation::CreatePage { page: base.pages[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeletePage
