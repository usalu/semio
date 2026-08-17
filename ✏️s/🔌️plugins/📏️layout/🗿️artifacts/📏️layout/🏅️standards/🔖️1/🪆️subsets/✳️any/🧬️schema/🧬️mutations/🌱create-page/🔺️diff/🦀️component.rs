//! 🔺 Diff constructor for `create-page`.

use super::mutation::CreatePage;
use crate::artifacts::layout::schema::diff::LayoutPagesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🌱️CreatePage
pub fn diff_create_page(payload: &CreatePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.pages.iter().any(|page| page.id == payload.page.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A page with id \"{}\" already exists.", payload.page.id), [payload.page.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { added: vec![payload.page.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🌱️CreatePage
