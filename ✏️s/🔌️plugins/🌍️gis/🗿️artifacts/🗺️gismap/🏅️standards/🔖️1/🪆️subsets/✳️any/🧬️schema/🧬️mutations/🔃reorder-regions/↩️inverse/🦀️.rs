//! ↩️ Inverse reconstruction for `reorder-regions` — reads the BASE position, never the diff.
use super::ReorderRegions;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo moves the feature back to its pre-reorder index, captured from `base` — missing target
/// returns `Vec::new()`.
pub fn inverse<Op>(payload: &ReorderRegions, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::reorder_regions::ReorderRegions>,
{
    let Some(current_index) = base.regions.iter().position(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![Op::from(ReorderRegions { id: payload.id.clone(), to_index: current_index })])
}
//#endregion 🔹Inverse
