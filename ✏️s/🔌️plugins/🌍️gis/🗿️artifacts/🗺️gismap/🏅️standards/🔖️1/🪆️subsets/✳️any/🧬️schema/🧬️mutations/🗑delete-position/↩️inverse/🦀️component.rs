//! ↩️ Inverse reconstruction for `delete-position` — reads the BASE item, never the diff.
use super::mutation::DeletePosition;
use crate::artifacts::gismap::mutations::create_position::mutation::CreatePosition;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo re-creates the feature at its pre-deletion index, captured from `base` — missing target
/// (already absent) returns `Vec::new()`, an empty inverse rather than a no-op sentinel mutation.
pub fn inverse(payload: &DeletePosition, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    let Some(index) = base.positions.iter().position(|feature| feature.id == payload.id) else {
        return Vec::new();
    };
    vec![GisMapMutation::CreatePosition(CreatePosition { index, item: base.positions[index].clone() })]
}
//#endregion 🔹Inverse
