//! 🔺 Diff constructor for `move-frame` — a `PagePatch.frame_patched` fragment carrying a
//! bounds-only `FramePatch`.

use super::mutation::MoveFrame;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};

//#region 🕹️MoveFrame
pub fn diff_move_frame(payload: &MoveFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if !page.frames.iter().any(|frame| frame.id() == payload.frame_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    }
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Frame \"{}\" position must be finite, got ({}, {}).", payload.frame_id, payload.new_x, payload.new_y), [payload.frame_id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch {
                    frame_patched: Some(PageFramePatched {
                        frame_id: payload.frame_id.clone(),
                        patch: FramePatch { x: Some(payload.new_x), y: Some(payload.new_y), ..Default::default() },
                    }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🕹️MoveFrame
