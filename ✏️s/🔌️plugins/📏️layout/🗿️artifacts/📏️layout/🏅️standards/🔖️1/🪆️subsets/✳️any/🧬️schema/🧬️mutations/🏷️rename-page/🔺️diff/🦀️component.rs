//! 🔺 Diff constructor for `rename-page`.

use super::mutation::RenamePage;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region 🏷️RenamePage
pub fn diff_rename_page(payload: &RenamePage, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { name: Some(payload.new_name.clone()), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🏷️RenamePage
