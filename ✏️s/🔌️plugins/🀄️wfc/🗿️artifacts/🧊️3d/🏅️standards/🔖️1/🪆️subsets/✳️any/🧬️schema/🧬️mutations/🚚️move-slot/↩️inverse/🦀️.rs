//! ↩️ Inverse for `MoveSlot` — moves the slot back to its BASE position (missing id ⇒ empty).

use crate::standards::v1::subsets::any::schema::mutations::move_slot::move_slot;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::MoveSlot, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::move_slot::MoveSlot>,
{
    Ok((|| {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    vec![move_slot(slot.id.clone(), slot.x, slot.y, slot.z)]

    })())
}
