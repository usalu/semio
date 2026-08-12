//! ↩ Inverse constructor for `create-page` — always undoes to `delete-page`.

use super::mutation::CreatePage;
use crate::artifacts::layout::mutations::{delete_page, LayoutMutation};
use crate::artifacts::layout::LayoutSnapshot;

//#region 🌱️CreatePage
pub fn inverse_create_page(payload: &CreatePage, _base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::DeletePage(delete_page::mutation::DeletePage { id: payload.page.id.clone() })]
}
//#endregion 🌱️CreatePage
