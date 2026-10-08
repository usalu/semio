//! ↩️ Inverse reconstruction for `replace-route-data` — reads the BASE payload, never the diff.
use super::ReplaceRouteData;
use crate::mutations::GisMapMutation;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base`'s prior `data` payload for this id — missing target returns `Vec::new()`.
pub fn inverse(payload: &ReplaceRouteData, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
    let Some(feature) = base.routes.iter().find(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![GisMapMutation::ReplaceRouteData(ReplaceRouteData { id: payload.id.clone(), new_data: feature.data.clone() })])
}
//#endregion 🔹Inverse
