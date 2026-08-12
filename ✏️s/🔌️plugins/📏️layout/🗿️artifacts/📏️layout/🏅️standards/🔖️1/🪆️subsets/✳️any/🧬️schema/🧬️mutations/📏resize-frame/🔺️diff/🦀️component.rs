//! 🔺 Diff constructor for `resize-frame`.

use super::mutation::ResizeFrame;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};

//#region 📏ResizeFrame
pub fn diff_resize_frame(payload: &ResizeFrame, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch {
                    frame_patched: Some(PageFramePatched {
                        frame_id: payload.frame_id.clone(),
                        patch: FramePatch { width: Some(payload.new_width), height: Some(payload.new_height), ..Default::default() },
                    }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 📏ResizeFrame
