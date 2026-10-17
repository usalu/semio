//! ↩️ Inverse for `DeleteSlot` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::connect_slots::connect_slots;
use crate::standards::v1::subsets::any::schema::mutations::create_slot::create_slot;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::DeleteSlot, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots> + From<crate::standards::v1::subsets::any::schema::mutations::create_slot::CreateSlot>,
{
    Ok((|| {
    let Some(slot) = base.slots.iter().find(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    let mut restore = vec![create_slot(slot.clone())];
    for edge in &base.edges {
        if edge.from_slot_id == payload.id || edge.to_slot_id == payload.id {
            restore.push(connect_slots(edge.clone()));
        }
    }
    restore.reverse();
    restore

    })())
}
