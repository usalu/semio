//! 🔺 Diff constructor for `change-page-width`.

use super::mutation::ChangePageWidth;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region ↔️ChangePageWidth
pub fn diff_change_page_width(payload: &ChangePageWidth, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { width: Some(payload.new_width), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion ↔️ChangePageWidth
