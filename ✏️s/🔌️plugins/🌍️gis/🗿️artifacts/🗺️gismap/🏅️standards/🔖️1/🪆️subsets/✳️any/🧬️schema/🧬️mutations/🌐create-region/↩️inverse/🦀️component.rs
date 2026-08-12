//! ↩️ Inverse reconstruction for `create-region` — undo is deleting the created feature.
use super::mutation::CreateRegion;
use crate::artifacts::gismap::mutations::delete_region::mutation::DeleteRegion;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo removes the feature this mutation created, addressed by its own id (captured from the
/// payload itself, not from `base` — a `create` has nothing to look up in pre-state).
pub fn inverse(payload: &CreateRegion, _base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    vec![GisMapMutation::DeleteRegion(DeleteRegion { id: payload.item.id.clone() })]
}
//#endregion 🔹Inverse
