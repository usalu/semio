//! ↩️ Inverse reconstruction for `remove-route-property` — reads the BASE payload, never the diff.
use super::RemoveRouteProperty;
use crate::mutations::{set_route_property::SetRouteProperty, GisMapMutation};
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo sets the removed property back with its BASE value, inserted before the key that followed it so the payload's
/// key order survives (appended when it was last) — a missing target, non-object payload or absent key returns `Vec::new()`.
pub fn inverse(payload: &RemoveRouteProperty, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
    let Some(entries) = base.routes.iter().find(|feature| feature.id == payload.feature).and_then(|feature| feature.data.as_object()) else {
        return Ok(Vec::new());
    };
    let Some(at) = entries.iter().position(|(key, _)| *key == payload.key) else {
        return Ok(Vec::new());
    };
    Ok(vec![GisMapMutation::SetRouteProperty(SetRouteProperty { feature: payload.feature.clone(), key: payload.key.clone(), value: entries[at].1.clone(), before: entries.get(at + 1).map(|(key, _)| key.clone()) })])
}
//#endregion 🔹Inverse
