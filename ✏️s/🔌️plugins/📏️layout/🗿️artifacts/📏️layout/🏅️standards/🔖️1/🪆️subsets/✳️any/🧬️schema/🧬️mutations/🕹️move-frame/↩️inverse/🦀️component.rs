//! ↩ Inverse constructor for `move-frame` — reconstructed from captured BASE state.

use super::mutation::MoveFrame;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 🕹️MoveFrame
pub fn inverse_move_frame(payload: &MoveFrame, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let bounds = frame.bounds();
    vec![LayoutMutation::MoveFrame(MoveFrame { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_x: bounds.x, new_y: bounds.y })]
}
//#endregion 🕹️MoveFrame
