//! 🔺 Diff constructor for `update-page-columns`.

use super::mutation::UpdatePageColumns;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region 🏛️UpdatePageColumns
pub fn diff_update_page_columns(payload: &UpdatePageColumns, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.id.clone(),
                patch: PagePatch { columns_count: Some(payload.count), columns_gutter: Some(payload.gutter), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🏛️UpdatePageColumns
