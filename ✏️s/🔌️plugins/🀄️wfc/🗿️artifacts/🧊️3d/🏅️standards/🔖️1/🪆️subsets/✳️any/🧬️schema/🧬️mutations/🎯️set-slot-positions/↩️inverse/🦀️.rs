//! ↩️ Inverse for `SetSlotPositions` — ONE `set-slot-positions` putting every slot the row moves back at its BASE
//! position; nothing when it moves nothing.

use crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::set_slot_positions;
use crate::schema::mutations::set_slot_positions::Wfc3dSlotPosition;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::SetSlotPositions, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::SetSlotPositions>,
{
    Ok((|| {
    if !payload.holds_invariants() {
        return Vec::new();
    }
    let positions: Vec<Wfc3dSlotPosition> = base
        .slots
        .iter()
        .filter(|slot| payload.positions.iter().any(|position| position.id == slot.id && (position.x, position.y, position.z) != (slot.x, slot.y, slot.z)))
        .map(|slot| Wfc3dSlotPosition { id: slot.id.clone(), x: slot.x, y: slot.y, z: slot.z })
        .collect();
    match positions.is_empty() {
        true => Vec::new(),
        false => vec![set_slot_positions(positions)],
    }

    })())
}
