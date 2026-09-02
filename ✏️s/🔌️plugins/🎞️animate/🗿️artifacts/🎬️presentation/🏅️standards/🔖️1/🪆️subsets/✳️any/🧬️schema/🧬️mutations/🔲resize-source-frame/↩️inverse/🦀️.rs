//! ↩️ Inverse reconstruction for `resize-source-frame` — reads the BASE frame, never the diff.
use super::ResizeSourceFrame;
use crate::artifacts::presentation::mutations::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source.frame` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ResizeSourceFrame, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (source, _) = crate::artifacts::presentation::presentation_working_scene(base);
    vec![PresentationMutation::ResizeSourceFrame(ResizeSourceFrame { new_frame: source.frame })]
}
//#endregion 🔹Inverse
