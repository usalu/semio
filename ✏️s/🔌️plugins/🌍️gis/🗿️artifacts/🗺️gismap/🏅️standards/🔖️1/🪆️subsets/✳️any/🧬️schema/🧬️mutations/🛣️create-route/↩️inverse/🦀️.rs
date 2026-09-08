//! ↩️ Inverse reconstruction for `create-route` — undo is deleting the created feature.
use super::CreateRoute;
use crate::mutations::delete_route::DeleteRoute;
use crate::mutations::GisMapMutation;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo removes the feature this mutation created, addressed by its own id (captured from the
/// payload itself, not from `base` — a `create` has nothing to look up in pre-state).
pub fn inverse(payload: &CreateRoute, _base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    vec![GisMapMutation::DeleteRoute(DeleteRoute { id: payload.item.id.clone() })]
}
//#endregion 🔹Inverse
