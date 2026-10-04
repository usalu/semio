//! ↩️ Inverse reconstruction for `change-imported-features` — reads the BASE value, never the diff.
use super::ChangeImportedFeatures;
use crate::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.imported_map` — captured from pre-state, never from the
/// applied diff.
pub fn inverse(_payload: &ChangeImportedFeatures, base: &GisTerrainSnapshot) -> Result<Vec<GisTerrainMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: base.imported_map.clone() })]

    })())
}
//#endregion 🔹Inverse
