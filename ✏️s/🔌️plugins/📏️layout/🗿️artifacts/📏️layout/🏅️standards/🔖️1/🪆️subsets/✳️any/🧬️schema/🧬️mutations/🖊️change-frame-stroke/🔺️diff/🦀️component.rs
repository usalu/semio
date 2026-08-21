//! 🔺 Diff constructor for `change-frame-stroke`.

use super::mutation::ChangeFrameStroke;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{Frame, FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};

//#region 🖊️ChangeFrameStroke
pub async fn diff_change_frame_stroke(payload: &ChangeFrameStroke, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    if let Frame::Rect { stroke, .. } = frame {
        if *stroke == payload.new_stroke {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Frame \"{}\" already has that stroke.", payload.frame_id));
        }
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { stroke: Some(payload.new_stroke), ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🖊️ChangeFrameStroke
