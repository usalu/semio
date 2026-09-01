//! ↩️ Inverse reconstruction for `reorder-positions` — reads the BASE position, never the diff.
use super::ReorderPositions;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo moves the feature back to its pre-reorder index, captured from `base` — missing target
/// returns `Vec::new()`.
pub fn inverse(payload: &ReorderPositions, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    let Some(current_index) = base.positions.iter().position(|feature| feature.id == payload.id) else {
        return Vec::new();
    };
    vec![GisMapMutation::ReorderPositions(ReorderPositions { id: payload.id.clone(), to_index: current_index })]
}
//#endregion 🔹Inverse
