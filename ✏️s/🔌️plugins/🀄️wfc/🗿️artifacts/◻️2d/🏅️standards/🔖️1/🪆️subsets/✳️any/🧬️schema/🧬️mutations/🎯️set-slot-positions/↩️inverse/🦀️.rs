//! ↩️ Inverse for `SetSlotPositions` — ONE `set-slot-positions` putting every slot the row moves back at its BASE
//! position; nothing when it moves nothing.

use crate::mutations::Wfc2dSlotPosition;
use crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::set_slot_positions;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::SetSlotPositions, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::SetSlotPositions>,
{
    Ok((|| {
    if !payload.holds_invariants() {
        return Vec::new();
    }
    let positions: Vec<Wfc2dSlotPosition> = base
        .slots
        .iter()
        .filter(|slot| payload.positions.iter().any(|position| position.id == slot.id && (position.x, position.y) != (slot.x, slot.y)))
        .map(|slot| Wfc2dSlotPosition { id: slot.id.clone(), x: slot.x, y: slot.y })
        .collect();
    match positions.is_empty() {
        true => Vec::new(),
        false => vec![set_slot_positions(positions)],
    }

    })())
}
