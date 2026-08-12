//! ↩ Inverse constructor for `change-frame-stroke` — reconstructed from captured BASE state. A
//! non-rect frame has nothing to undo (the field-patch was a no-op).

use super::mutation::ChangeFrameStroke;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{Frame, LayoutSnapshot};

//#region 🖊️ChangeFrameStroke
pub fn inverse_change_frame_stroke(payload: &ChangeFrameStroke, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Rect { stroke, .. } = frame else {
        return Vec::new();
    };
    vec![LayoutMutation::ChangeFrameStroke(ChangeFrameStroke { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_stroke: *stroke })]
}
//#endregion 🖊️ChangeFrameStroke
