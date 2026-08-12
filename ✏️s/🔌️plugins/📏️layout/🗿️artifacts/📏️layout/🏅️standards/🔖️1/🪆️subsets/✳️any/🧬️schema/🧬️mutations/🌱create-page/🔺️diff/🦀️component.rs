//! 🔺 Diff constructor for `create-page`.

use super::mutation::CreatePage;
use crate::artifacts::layout::schema::diff::LayoutPagesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🌱️CreatePage
pub fn diff_create_page(payload: &CreatePage, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { pages: Some(LayoutPagesDelta { added: vec![payload.page.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🌱️CreatePage
