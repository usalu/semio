//! ↩️ Inverse reconstruction for `reorder-routes` — reads the BASE position, never the diff.
use super::ReorderRoutes;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo moves the feature back to its pre-reorder index, captured from `base` — missing target
/// returns `Vec::new()`.
pub fn inverse<Op>(payload: &ReorderRoutes, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::reorder_routes::ReorderRoutes>,
{
    let Some(current_index) = base.routes.iter().position(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![Op::from(ReorderRoutes { id: payload.id.clone(), to_index: current_index })])
}
//#endregion 🔹Inverse
