//! 🔺 Diff constructor for `delete-frame` — a `PagePatch.frame_removed` fragment.

use super::mutation::DeleteFrame;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region ➖️DeleteFrame
pub fn diff_delete_frame(payload: &DeleteFrame, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_removed: Some(payload.frame_id.clone()), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion ➖️DeleteFrame
