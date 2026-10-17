//! ↩️ Inverse for `DeleteSlot` — recreates the slot AND every incident edge the delete cascaded
//! away, each as its own point mutation of this same vocabulary, at its own BASE position (missing
//! id ⇒ empty: nothing to undo).

use crate::standards::v1::subsets::any::schema::mutations::connect_slots::connect_slots;
use crate::standards::v1::subsets::any::schema::mutations::create_slot::create_slot;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::DeleteSlot, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_slots::ConnectSlots> + From<crate::standards::v1::subsets::any::schema::mutations::create_slot::CreateSlot>,
{
    Ok((|| {
    let Some(index) = base.slots.iter().position(|slot| slot.id == payload.id) else {
        return Vec::new();
    };
    let mut restore = vec![create_slot(index, base.slots[index].clone())];
    for (edge_index, edge) in base.edges.iter().enumerate() {
        if edge.from_slot_id == payload.id || edge.to_slot_id == payload.id {
            restore.push(connect_slots(edge_index, edge.clone()));
        }
    }
    restore.reverse();
    restore

    })())
}
