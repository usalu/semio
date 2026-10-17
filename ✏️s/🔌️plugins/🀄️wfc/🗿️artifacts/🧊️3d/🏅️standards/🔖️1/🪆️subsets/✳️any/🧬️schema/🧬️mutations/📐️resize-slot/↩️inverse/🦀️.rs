//! ↩️ Inverse for `ResizeSlot` — restores the slot's BASE extent (missing id ⇒ empty).

use crate::standards::v1::subsets::any::schema::mutations::resize_slot::resize_slot;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::ResizeSlot, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_slot::ResizeSlot>,
{
    Ok((|| {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    vec![resize_slot(slot.id.clone(), slot.width, slot.height, slot.depth)]

    })())
}
