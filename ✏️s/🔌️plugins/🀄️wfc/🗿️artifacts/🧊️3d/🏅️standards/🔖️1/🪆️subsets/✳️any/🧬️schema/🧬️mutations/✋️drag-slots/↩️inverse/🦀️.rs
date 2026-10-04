//! ↩️ Inverse for `DragSlots` — ONE absolute `set-slot-positions` putting every moved slot back at its BASE position
//! (never a negated offset, which rounding would not restore exactly); nothing when the drag moves nothing.

use crate::mutations::{set_slot_positions, Wfc3dMutation, Wfc3dSlotPosition};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::DragSlots, base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if !payload.holds_invariants() || payload.is_zero() {
        return Vec::new();
    }
    let positions: Vec<Wfc3dSlotPosition> = base.slots.iter().filter(|slot| payload.targets.contains(&slot.id)).map(|slot| Wfc3dSlotPosition { id: slot.id.clone(), x: slot.x, y: slot.y, z: slot.z }).collect();
    match positions.is_empty() {
        true => Vec::new(),
        false => vec![set_slot_positions(positions)],
    }

    })())
}
