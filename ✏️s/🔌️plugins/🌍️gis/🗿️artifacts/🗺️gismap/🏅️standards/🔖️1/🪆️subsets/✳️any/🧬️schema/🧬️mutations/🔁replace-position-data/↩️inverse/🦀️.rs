//! ↩️ Inverse reconstruction for `replace-position-data` — reads the BASE payload, never the diff.
use super::ReplacePositionData;
use crate::mutations::GisMapMutation;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base`'s prior `data` payload for this id — missing target returns `Vec::new()`.
pub fn inverse(payload: &ReplacePositionData, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
    let Some(feature) = base.positions.iter().find(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![GisMapMutation::ReplacePositionData(ReplacePositionData { id: payload.id.clone(), new_data: feature.data.clone() })])
}
//#endregion 🔹Inverse
