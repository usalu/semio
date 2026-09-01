//! ↩️ Inverse reconstruction for `replace-position-data` — reads the BASE payload, never the diff.
use super::ReplacePositionData;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base`'s prior `data` payload for this id — missing target returns `Vec::new()`.
pub fn inverse(payload: &ReplacePositionData, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    let Some(feature) = base.positions.iter().find(|feature| feature.id == payload.id) else {
        return Vec::new();
    };
    vec![GisMapMutation::ReplacePositionData(ReplacePositionData { id: payload.id.clone(), new_data: feature.data.clone() })]
}
//#endregion 🔹Inverse
