//! 🔺️ Sparse diff construction for the `replace-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::ReplaceEquipment;
use crate::diff::ProgramEquipmentDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceEquipment, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.equipment.header.id;
    let Some(position) = base.equipment.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No equipment exists with this id.", [id.0.clone()]);
    };
    if base.equipment[position] == payload.equipment {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This equipment already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramEquipmentDelta::removal(&base.equipment, position);
    delta.absorb(ProgramEquipmentDelta::insertion(position, payload.equipment.clone()));
    protocol::MutationOutcome::new(ProgramDiff { equipment: Some(delta), ..Default::default() })
}
