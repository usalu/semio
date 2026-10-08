//! ↩️ Inverse reconstruction for `reorder-regions` — reads the BASE position, never the diff.
use super::ReorderRegions;
use crate::mutations::GisMapMutation;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo moves the feature back to its pre-reorder index, captured from `base` — missing target
/// returns `Vec::new()`.
pub fn inverse(payload: &ReorderRegions, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
    let Some(current_index) = base.regions.iter().position(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![GisMapMutation::ReorderRegions(ReorderRegions { id: payload.id.clone(), to_index: current_index })])
}
//#endregion 🔹Inverse
