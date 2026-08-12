//! ↩ Inverse constructor for `resize-frame` — reconstructed from captured BASE state.

use super::mutation::ResizeFrame;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 📏ResizeFrame
pub fn inverse_resize_frame(payload: &ResizeFrame, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let bounds = frame.bounds();
    vec![LayoutMutation::ResizeFrame(ResizeFrame { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_width: bounds.width, new_height: bounds.height })]
}
//#endregion 📏ResizeFrame
