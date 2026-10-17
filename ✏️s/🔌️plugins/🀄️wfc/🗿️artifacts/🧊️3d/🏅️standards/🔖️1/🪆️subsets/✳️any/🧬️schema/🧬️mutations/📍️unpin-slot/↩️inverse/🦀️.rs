//! ↩️ Inverse for `UnpinSlot` — re-pins the slot to the tile it named in BASE (no pin ⇒ empty).

use crate::standards::v1::subsets::any::schema::mutations::pin_slot::pin_slot;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::UnpinSlot, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
{
    Ok((|| {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    match &slot.pinned_tile_id {
        Some(previous) => vec![pin_slot(slot.id.clone(), previous.clone())],
        None => Vec::new(),
    }

    })())
}
