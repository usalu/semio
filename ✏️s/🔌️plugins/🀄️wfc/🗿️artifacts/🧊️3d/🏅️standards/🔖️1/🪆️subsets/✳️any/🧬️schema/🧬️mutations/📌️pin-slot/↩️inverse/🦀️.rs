//! ↩️ Inverse for `PinSlot` — restores the slot's PRIOR pin, or unpins it when there was none.

use crate::standards::v1::subsets::any::schema::mutations::pin_slot::pin_slot;
use crate::standards::v1::subsets::any::schema::mutations::unpin_slot::unpin_slot;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::PinSlot, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_slot::UnpinSlot>,
{
    Ok((|| {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    match &slot.pinned_tile_id {
        Some(previous) => vec![pin_slot(slot.id.clone(), previous.clone())],
        None => vec![unpin_slot(slot.id.clone())],
    }

    })())
}
