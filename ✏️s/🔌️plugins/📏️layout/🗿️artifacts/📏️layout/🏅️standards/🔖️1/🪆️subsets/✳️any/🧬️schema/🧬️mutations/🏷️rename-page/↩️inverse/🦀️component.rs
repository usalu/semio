//! ↩ Inverse constructor for `rename-page` — reconstructed from captured BASE state.

use super::mutation::RenamePage;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 🏷️RenamePage
pub fn inverse_rename_page(payload: &RenamePage, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::RenamePage(RenamePage { id: payload.id.clone(), new_name: page.name.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🏷️RenamePage
