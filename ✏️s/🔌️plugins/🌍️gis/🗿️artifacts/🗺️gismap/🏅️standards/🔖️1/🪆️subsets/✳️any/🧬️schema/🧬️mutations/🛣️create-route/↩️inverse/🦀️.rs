//! ↩️ Inverse reconstruction for `create-route` — undo is deleting the created feature.
use super::CreateRoute;
use crate::mutations::delete_route::DeleteRoute;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo removes the feature this mutation created, addressed by its own id (captured from the
/// payload itself, not from `base` — a `create` has nothing to look up in pre-state).
pub fn inverse<Op>(payload: &CreateRoute, _base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_route::DeleteRoute>,
{
    Ok(vec![Op::from(DeleteRoute { id: payload.item.id.clone() })])
}
//#endregion 🔹Inverse
