//! ↩️ Inverse reconstruction for `reorder-regions` — reads the BASE position, never the diff.
use super::mutation::ReorderRegions;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo moves the feature back to its pre-reorder index, captured from `base` — missing target
/// returns `Vec::new()`.
pub async fn inverse(payload: &ReorderRegions, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    let Some(current_index) = base.regions.iter().position(|feature| feature.id == payload.id) else {
        return Vec::new();
    };
    vec![GisMapMutation::ReorderRegions(ReorderRegions { id: payload.id.clone(), to_index: current_index })]
}
//#endregion 🔹Inverse
