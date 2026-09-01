//! ↩️ Inverse reconstruction for `resize-source-frame` — reads the BASE frame, never the diff.
use super::ResizeSourceFrame;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source.frame` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ResizeSourceFrame, base: &PresentSnapshot) -> Vec<PresentMutation> {
    let (source, _) = crate::artifacts::present::present_working_scene(base);
    vec![PresentMutation::ResizeSourceFrame(ResizeSourceFrame { new_frame: source.frame })]
}
//#endregion 🔹Inverse
