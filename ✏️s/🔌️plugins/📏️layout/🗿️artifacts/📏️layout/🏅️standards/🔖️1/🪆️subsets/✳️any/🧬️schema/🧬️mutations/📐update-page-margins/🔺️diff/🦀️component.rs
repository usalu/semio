//! 🔺 Diff constructor for `update-page-margins`.

use super::mutation::UpdatePageMargins;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region 📐UpdatePageMargins
pub fn diff_update_page_margins(payload: &UpdatePageMargins, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.id.clone(),
                patch: PagePatch {
                    margin_top: Some(payload.top),
                    margin_right: Some(payload.right),
                    margin_bottom: Some(payload.bottom),
                    margin_left: Some(payload.left),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 📐UpdatePageMargins
