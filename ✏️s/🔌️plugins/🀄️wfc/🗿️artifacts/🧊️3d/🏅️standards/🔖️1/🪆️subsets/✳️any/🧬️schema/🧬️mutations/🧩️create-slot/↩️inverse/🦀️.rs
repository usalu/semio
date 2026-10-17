//! ↩️ Inverse for `CreateSlot` — the `delete-slot` of the id it created (the payload carries the id,
//! so no BASE lookup is needed to know what to undo).

use crate::standards::v1::subsets::any::schema::mutations::delete_slot::delete_slot;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::CreateSlot, _base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_slot::DeleteSlot>,
{
    Ok((|| {
    vec![delete_slot(payload.slot.id.clone())]

    })())
}
