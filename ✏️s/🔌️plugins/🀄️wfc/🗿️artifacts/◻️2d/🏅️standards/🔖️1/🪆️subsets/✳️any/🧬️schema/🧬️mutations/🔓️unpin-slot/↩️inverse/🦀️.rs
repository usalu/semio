//! ↩️ Inverse for `UnpinSlot` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::pin_slot::pin_slot;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::UnpinSlot, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
{
    Ok((|| {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    match &slot.pinned_tile_id {
        Some(previous) => vec![pin_slot(payload.id.clone(), previous.clone())],
        None => Vec::new(),
    }

    })())
}
