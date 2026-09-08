//! ↩️ Inverse reconstruction for `resize-source-frame` — reads the BASE frame, never the diff.
use super::ResizeSourceFrame;
use crate::mutations::PresentationMutation;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source.frame` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ResizeSourceFrame, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (source, _) = crate::presentation_working_scene(base);
    vec![PresentationMutation::ResizeSourceFrame(ResizeSourceFrame { new_frame: source.frame })]
}
//#endregion 🔹Inverse
