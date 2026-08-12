//! 🔺 Diff constructor for `create-frame` — a `PagePatch.frame_added` fragment nested under the
//! target page's `LayoutPagesDelta` patch entry (never apply-then-capture).

use super::mutation::CreateFrame;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PageFrameAdded, PagePatch};

//#region ➕️CreateFrame
pub fn diff_create_frame(payload: &CreateFrame, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch {
                    frame_added: Some(PageFrameAdded { frame: payload.frame.clone(), index: payload.index, layer_id: payload.layer_id.clone() }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion ➕️CreateFrame
