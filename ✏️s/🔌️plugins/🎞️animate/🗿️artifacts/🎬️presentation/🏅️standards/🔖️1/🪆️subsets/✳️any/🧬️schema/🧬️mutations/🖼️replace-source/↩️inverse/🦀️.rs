//! ↩️ Inverse reconstruction for `replace-source` — reads the BASE source, never the diff.
use super::ReplaceSource;
use crate::artifacts::presentation::mutations::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ReplaceSource, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (source, _) = crate::artifacts::presentation::presentation_working_scene(base);
    vec![PresentationMutation::ReplaceSource(ReplaceSource { new_source: source })]
}
//#endregion 🔹Inverse
