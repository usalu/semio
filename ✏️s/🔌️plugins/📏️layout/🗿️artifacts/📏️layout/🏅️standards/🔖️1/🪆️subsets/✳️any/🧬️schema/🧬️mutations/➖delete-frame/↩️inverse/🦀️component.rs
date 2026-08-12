//! ↩ Inverse constructor for `delete-frame` — captures the removed frame's full payload, its
//! position within the page, and which layer (if any) referenced it.

use super::mutation::DeleteFrame;
use crate::artifacts::layout::mutations::{create_frame, LayoutMutation};
use crate::artifacts::layout::LayoutSnapshot;

//#region ➖️DeleteFrame
pub fn inverse_delete_frame(payload: &DeleteFrame, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(index) = page.frames.iter().position(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let frame = page.frames[index].clone();
    let layer_id = page.layers.iter().find(|layer| layer.object_ids.iter().any(|id| id == &payload.frame_id)).map(|layer| layer.id.clone());
    vec![LayoutMutation::CreateFrame(create_frame::mutation::CreateFrame { page_id: payload.page_id.clone(), frame, index: Some(index), layer_id })]
}
//#endregion ➖️DeleteFrame
