//! ↩️ Inverse reconstruction for `replace-source` — reads the BASE source, never the diff.
use super::ReplaceSource;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ReplaceSource, base: &PresentSnapshot) -> Vec<PresentMutation> {
    let (source, _) = crate::artifacts::present::present_working_scene(base);
    vec![PresentMutation::ReplaceSource(ReplaceSource { new_source: source })]
}
//#endregion 🔹Inverse
