//! ↩ Inverse constructor for `change-frame-columns` — reconstructed from captured BASE state. A
//! non-text frame has nothing to undo (the field-patch was a no-op).

use super::mutation::ChangeFrameColumns;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{Frame, LayoutSnapshot};

//#region 🔢ChangeFrameColumns
pub fn inverse_change_frame_columns(payload: &ChangeFrameColumns, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Text { columns, .. } = frame else {
        return Vec::new();
    };
    vec![LayoutMutation::ChangeFrameColumns(ChangeFrameColumns { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_columns: *columns })]
}
//#endregion 🔢ChangeFrameColumns
