//! ↩️ Inverse reconstruction for `replace-source` — reads the BASE source, never the diff.
use super::ReplaceSource;
use crate::mutations::PresentationMutation;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ReplaceSource, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (source, _) = crate::presentation_working_scene(base);
    vec![PresentationMutation::ReplaceSource(ReplaceSource { new_source: source })]
}
//#endregion 🔹Inverse
