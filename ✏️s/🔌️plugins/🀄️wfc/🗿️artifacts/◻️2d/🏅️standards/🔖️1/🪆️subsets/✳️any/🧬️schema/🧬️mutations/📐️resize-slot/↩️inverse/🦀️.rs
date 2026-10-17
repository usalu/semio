//! ↩️ Inverse for `ResizeSlot` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::resize_slot::resize_slot;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::ResizeSlot, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_slot::ResizeSlot>,
{
    Ok((|| {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    vec![resize_slot(payload.id.clone(), slot.width, slot.height)]

    })())
}
