//! ↩ Inverse constructor for `create-frame` — always undoes to `delete-frame` (matches the
//! pre-migration `AddFrame`'s inverse, which never inspected `base`).

use super::mutation::CreateFrame;
use crate::artifacts::layout::mutations::{delete_frame, LayoutMutation};
use crate::artifacts::layout::LayoutSnapshot;

//#region ➕️CreateFrame
pub fn inverse_create_frame(payload: &CreateFrame, _base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::DeleteFrame(delete_frame::mutation::DeleteFrame { page_id: payload.page_id.clone(), frame_id: payload.frame.id().to_string() })]
}
//#endregion ➕️CreateFrame
