//! ↩️ Inverse reconstruction for `delete-route` — reads the BASE item, never the diff.
use super::DeleteRoute;
use crate::artifacts::gismap::mutations::create_route::CreateRoute;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo re-creates the feature at its pre-deletion index, captured from `base` — missing target
/// (already absent) returns `Vec::new()`, an empty inverse rather than a no-op sentinel mutation.
pub fn inverse(payload: &DeleteRoute, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    let Some(index) = base.routes.iter().position(|feature| feature.id == payload.id) else {
        return Vec::new();
    };
    vec![GisMapMutation::CreateRoute(CreateRoute { index, item: base.routes[index].clone() })]
}
//#endregion 🔹Inverse
