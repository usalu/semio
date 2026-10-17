//! ↩️ Inverse reconstruction for `reorder-positions` — reads the BASE position, never the diff.
use super::ReorderPositions;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo moves the feature back to its pre-reorder index, captured from `base` — missing target
/// returns `Vec::new()`.
pub fn inverse<Op>(payload: &ReorderPositions, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::reorder_positions::ReorderPositions>,
{
    let Some(current_index) = base.positions.iter().position(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![Op::from(ReorderPositions { id: payload.id.clone(), to_index: current_index })])
}
//#endregion 🔹Inverse
