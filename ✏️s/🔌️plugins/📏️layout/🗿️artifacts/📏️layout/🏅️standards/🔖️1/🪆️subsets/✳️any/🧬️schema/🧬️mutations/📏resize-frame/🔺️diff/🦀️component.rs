//! 🔺 Diff constructor for `resize-frame`.

use super::mutation::ResizeFrame;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};

//#region 📏ResizeFrame
pub async fn diff_resize_frame(payload: &ResizeFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if !page.frames.iter().any(|frame| frame.id() == payload.frame_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    }
    if !payload.new_width.is_finite() || !payload.new_height.is_finite() || payload.new_width <= 0.0 || payload.new_height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Frame \"{}\" size must be finite and positive, got ({}, {}).", payload.frame_id, payload.new_width, payload.new_height), [payload.frame_id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { width: Some(payload.new_width), height: Some(payload.new_height), ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 📏ResizeFrame
