//! ↩ Inverse constructor for `change-frame-wrap-mode` — reconstructed from captured BASE state. A
//! non-text frame has nothing to undo (the field-patch was a no-op).

use super::mutation::ChangeFrameWrapMode;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{Frame, LayoutSnapshot};

//#region 🔤ChangeFrameWrapMode
pub fn inverse_change_frame_wrap_mode(payload: &ChangeFrameWrapMode, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Text { wrap_mode, .. } = frame else {
        return Vec::new();
    };
    vec![LayoutMutation::ChangeFrameWrapMode(ChangeFrameWrapMode { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_wrap_mode: wrap_mode.clone() })]
}
//#endregion 🔤ChangeFrameWrapMode
