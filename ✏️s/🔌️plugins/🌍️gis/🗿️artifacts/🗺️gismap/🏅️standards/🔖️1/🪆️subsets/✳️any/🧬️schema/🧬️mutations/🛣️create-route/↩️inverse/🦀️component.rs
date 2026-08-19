//! ↩️ Inverse reconstruction for `create-route` — undo is deleting the created feature.
use super::mutation::CreateRoute;
use crate::artifacts::gismap::mutations::delete_route::mutation::DeleteRoute;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo removes the feature this mutation created, addressed by its own id (captured from the
/// payload itself, not from `base` — a `create` has nothing to look up in pre-state).
pub async fn inverse(payload: &CreateRoute, _base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    vec![GisMapMutation::DeleteRoute(DeleteRoute { id: payload.item.id.clone() })]
}
//#endregion 🔹Inverse
