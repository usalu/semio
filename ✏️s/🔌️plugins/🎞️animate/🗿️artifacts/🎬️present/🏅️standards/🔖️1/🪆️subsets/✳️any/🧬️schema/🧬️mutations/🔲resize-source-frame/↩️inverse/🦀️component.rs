//! ↩️ Inverse reconstruction for `resize-source-frame` — reads the BASE frame, never the diff.
use super::mutation::ResizeSourceFrame;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source.frame` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ResizeSourceFrame, base: &PresentSnapshot) -> Vec<PresentMutation> {
    vec![PresentMutation::ResizeSourceFrame(ResizeSourceFrame { new_frame: base.source.frame.clone() })]
}
//#endregion 🔹Inverse
