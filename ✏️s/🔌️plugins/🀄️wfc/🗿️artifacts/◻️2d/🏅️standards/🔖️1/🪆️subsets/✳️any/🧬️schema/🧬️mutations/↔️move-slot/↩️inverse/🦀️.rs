//! ↩️ Inverse for `MoveSlot` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::move_slot::move_slot;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::MoveSlot, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::move_slot::MoveSlot>,
{
    Ok((|| {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    vec![move_slot(payload.id.clone(), slot.x, slot.y)]

    })())
}
