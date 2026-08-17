//! 🔺 Diff constructor for `change-frame-fill`.

use super::mutation::ChangeFrameFill;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{Frame, FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};

//#region 🎨ChangeFrameFill
pub fn diff_change_frame_fill(payload: &ChangeFrameFill, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    if let Frame::Rect { fill, .. } = frame {
        if *fill == payload.new_fill {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Frame \"{}\" already has that fill.", payload.frame_id));
        }
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch {
                    frame_patched: Some(PageFramePatched {
                        frame_id: payload.frame_id.clone(),
                        patch: FramePatch { fill: Some(payload.new_fill), ..Default::default() },
                    }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🎨ChangeFrameFill
