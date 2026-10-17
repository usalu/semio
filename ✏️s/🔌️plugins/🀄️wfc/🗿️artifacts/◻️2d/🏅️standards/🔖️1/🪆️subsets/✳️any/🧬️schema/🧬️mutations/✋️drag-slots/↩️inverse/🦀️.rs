//! ↩️ Inverse for `DragSlots` — ONE absolute `set-slot-positions` putting every moved slot back at its BASE position
//! (never a negated offset, which rounding would not restore exactly); nothing when the drag moves nothing.

use crate::mutations::Wfc2dSlotPosition;
use crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::set_slot_positions;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(payload: &super::DragSlots, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::SetSlotPositions>,
{
    Ok((|| {
    if !payload.holds_invariants() || (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    let positions: Vec<Wfc2dSlotPosition> = base.slots.iter().filter(|slot| payload.targets.contains(&slot.id)).map(|slot| Wfc2dSlotPosition { id: slot.id.clone(), x: slot.x, y: slot.y }).collect();
    match positions.is_empty() {
        true => Vec::new(),
        false => vec![set_slot_positions(positions)],
    }

    })())
}
