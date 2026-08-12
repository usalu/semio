//! ↩️ Inverse reconstruction for `replace-source` — reads the BASE source, never the diff.
use super::mutation::ReplaceSource;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.source` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ReplaceSource, base: &PresentSnapshot) -> Vec<PresentMutation> {
    vec![PresentMutation::ReplaceSource(ReplaceSource { new_source: base.source.clone() })]
}
//#endregion 🔹Inverse
