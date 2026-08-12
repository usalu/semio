//! ↩️ Inverse reconstruction for `replace-region-data` — reads the BASE payload, never the diff.
use super::mutation::ReplaceRegionData;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base`'s prior `data` payload for this id — missing target returns `Vec::new()`.
pub fn inverse(payload: &ReplaceRegionData, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    let Some(feature) = base.regions.iter().find(|feature| feature.id == payload.id) else {
        return Vec::new();
    };
    vec![GisMapMutation::ReplaceRegionData(ReplaceRegionData { id: payload.id.clone(), new_data: feature.data.clone() })]
}
//#endregion 🔹Inverse
