//! ↩️ Inverse reconstruction for `change-exaggeration` — reads the BASE value, never the diff.
use super::mutation::ChangeExaggeration;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.exaggeration` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ChangeExaggeration, base: &GisTerrainSnapshot) -> Vec<GisTerrainMutation> {
    vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: base.exaggeration })]
}
//#endregion 🔹Inverse
