//! ↩️ Inverse reconstruction for `create-position` — undo is deleting the created feature.
use super::mutation::CreatePosition;
use crate::artifacts::gismap::mutations::delete_position::mutation::DeletePosition;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo removes the feature this mutation created, addressed by its own id (captured from the
/// payload itself, not from `base` — a `create` has nothing to look up in pre-state).
pub fn inverse(payload: &CreatePosition, _base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    vec![GisMapMutation::DeletePosition(DeletePosition { id: payload.item.id.clone() })]
}
//#endregion 🔹Inverse
