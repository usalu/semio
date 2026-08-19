//! ↩ Inverse constructor for `change-frame-fill` — reconstructed from captured BASE state. A
//! non-rect frame has nothing to undo (the field-patch was a no-op).

use super::mutation::ChangeFrameFill;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{Frame, LayoutSnapshot};

//#region 🎨ChangeFrameFill
pub async fn inverse_change_frame_fill(payload: &ChangeFrameFill, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Rect { fill, .. } = frame else {
        return Vec::new();
    };
    vec![LayoutMutation::ChangeFrameFill(ChangeFrameFill { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_fill: *fill })]
}
//#endregion 🎨ChangeFrameFill
