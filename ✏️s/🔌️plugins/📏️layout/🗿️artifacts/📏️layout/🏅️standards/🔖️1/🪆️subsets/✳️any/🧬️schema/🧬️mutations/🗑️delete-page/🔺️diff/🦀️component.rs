//! 🔺 Diff constructor for `delete-page`.

use super::mutation::DeletePage;
use crate::artifacts::layout::schema::diff::LayoutPagesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🗑️DeletePage
pub fn diff_delete_page(payload: &DeletePage, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { pages: Some(LayoutPagesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🗑️DeletePage
