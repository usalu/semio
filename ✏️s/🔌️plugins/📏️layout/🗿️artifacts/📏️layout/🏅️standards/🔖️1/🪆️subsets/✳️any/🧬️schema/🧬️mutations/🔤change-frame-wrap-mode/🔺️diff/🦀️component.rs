//! 🔺 Diff constructor for `change-frame-wrap-mode`.

use super::mutation::ChangeFrameWrapMode;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};

//#region 🔤ChangeFrameWrapMode
pub fn diff_change_frame_wrap_mode(payload: &ChangeFrameWrapMode, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch {
                    frame_patched: Some(PageFramePatched {
                        frame_id: payload.frame_id.clone(),
                        patch: FramePatch { wrap_mode: Some(payload.new_wrap_mode.clone()), ..Default::default() },
                    }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔤ChangeFrameWrapMode
