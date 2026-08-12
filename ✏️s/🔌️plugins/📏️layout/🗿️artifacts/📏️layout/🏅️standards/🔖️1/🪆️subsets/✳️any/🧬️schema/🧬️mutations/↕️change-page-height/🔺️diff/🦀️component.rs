//! 🔺 Diff constructor for `change-page-height`.

use super::mutation::ChangePageHeight;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region ↕️ChangePageHeight
pub fn diff_change_page_height(payload: &ChangePageHeight, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { height: Some(payload.new_height), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion ↕️ChangePageHeight
