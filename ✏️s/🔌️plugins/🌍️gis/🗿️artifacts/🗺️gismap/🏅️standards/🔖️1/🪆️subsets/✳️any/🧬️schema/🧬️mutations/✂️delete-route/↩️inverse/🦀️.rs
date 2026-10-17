//! ↩️ Inverse reconstruction for `delete-route` — reads the BASE item, never the diff.
use super::DeleteRoute;
use crate::mutations::create_route::CreateRoute;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo re-creates the feature at its pre-deletion index, captured from `base` — missing target
/// (already absent) returns `Vec::new()`, an empty inverse rather than a no-op sentinel mutation.
pub fn inverse<Op>(payload: &DeleteRoute, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_route::CreateRoute>,
{
    let Some(index) = base.routes.iter().position(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![Op::from(CreateRoute { index, item: base.routes[index].clone() })])
}
//#endregion 🔹Inverse
