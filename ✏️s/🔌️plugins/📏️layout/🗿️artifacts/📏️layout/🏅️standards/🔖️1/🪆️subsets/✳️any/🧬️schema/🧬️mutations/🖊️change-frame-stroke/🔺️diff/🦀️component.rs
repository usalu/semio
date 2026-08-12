//! 🔺 Diff constructor for `change-frame-stroke`.

use super::mutation::ChangeFrameStroke;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};

//#region 🖊️ChangeFrameStroke
pub fn diff_change_frame_stroke(payload: &ChangeFrameStroke, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch {
                    frame_patched: Some(PageFramePatched {
                        frame_id: payload.frame_id.clone(),
                        patch: FramePatch { stroke: Some(payload.new_stroke), ..Default::default() },
                    }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🖊️ChangeFrameStroke
