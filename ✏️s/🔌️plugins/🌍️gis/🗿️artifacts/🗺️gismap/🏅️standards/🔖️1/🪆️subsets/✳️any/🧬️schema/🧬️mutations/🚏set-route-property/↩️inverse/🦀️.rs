//! ↩️ Inverse reconstruction for `set-route-property` — reads the BASE payload, never the diff.
use super::SetRouteProperty;
use crate::mutations::{remove_route_property::RemoveRouteProperty, set_route_property::SetRouteProperty as SetProperty, GisMapMutation};
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores the displaced value in place when the key existed, and removes the key when the set created it —
/// a missing target or a non-object payload returns `Vec::new()`.
pub fn inverse(payload: &SetRouteProperty, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
    let Some(entries) = base.routes.iter().find(|feature| feature.id == payload.feature).and_then(|feature| feature.data.as_object()) else {
        return Ok(Vec::new());
    };
    Ok(match entries.iter().find(|(key, _)| *key == payload.key) {
        Some((_, previous)) if *previous == payload.value => Vec::new(),
        Some((_, previous)) => vec![GisMapMutation::SetRouteProperty(SetProperty { feature: payload.feature.clone(), key: payload.key.clone(), value: previous.clone(), before: None })],
        None => vec![GisMapMutation::RemoveRouteProperty(RemoveRouteProperty { feature: payload.feature.clone(), key: payload.key.clone() })],
    })
}
//#endregion 🔹Inverse
