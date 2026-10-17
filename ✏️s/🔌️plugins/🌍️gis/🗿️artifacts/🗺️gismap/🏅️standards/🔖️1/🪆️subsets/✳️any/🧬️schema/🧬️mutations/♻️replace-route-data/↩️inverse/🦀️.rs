//! ↩️ Inverse reconstruction for `replace-route-data` — reads the BASE payload, never the diff.
use super::ReplaceRouteData;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base`'s prior `data` payload for this id — missing target returns `Vec::new()`.
pub fn inverse<Op>(payload: &ReplaceRouteData, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_route_data::ReplaceRouteData>,
{
    let Some(feature) = base.routes.iter().find(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![Op::from(ReplaceRouteData { id: payload.id.clone(), new_data: feature.data.clone() })])
}
//#endregion 🔹Inverse
