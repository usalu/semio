//! ↩️ Inverse reconstruction for `change-exaggeration` — reads the BASE value, never the diff.
use super::ChangeExaggeration;
use crate::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.exaggeration` — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ChangeExaggeration, base: &GisTerrainSnapshot) -> Result<Vec<GisTerrainMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: base.exaggeration })]

    })())
}
//#endregion 🔹Inverse
