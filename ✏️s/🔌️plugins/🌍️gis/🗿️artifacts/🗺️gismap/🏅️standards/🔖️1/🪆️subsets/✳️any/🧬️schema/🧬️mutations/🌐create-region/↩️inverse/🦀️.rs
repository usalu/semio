//! ↩️ Inverse reconstruction for `create-region` — undo is deleting the created feature.
use super::CreateRegion;
use crate::mutations::delete_region::DeleteRegion;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo removes the feature this mutation created, addressed by its own id (captured from the
/// payload itself, not from `base` — a `create` has nothing to look up in pre-state).
pub fn inverse<Op>(payload: &CreateRegion, _base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_region::DeleteRegion>,
{
    Ok(vec![Op::from(DeleteRegion { id: payload.item.id.clone() })])
}
//#endregion 🔹Inverse
