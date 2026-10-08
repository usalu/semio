//! 🔺️ Sparse diff construction for the `replace-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::ReplaceEquipment;
use crate::diff::ProgramEquipmentDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceEquipment, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.equipment.header.id;
    let Some(position) = base.equipment.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No equipment exists with this id.", [id.0.clone()]);
    };
    if base.equipment[position] == payload.equipment {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This equipment already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.equipment.len()).then(|| base.equipment.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { equipment: Some(ProgramEquipmentDelta { removed: vec![id.0.clone()], added: vec![payload.equipment.clone()], reordered, ..Default::default() }), ..Default::default() })
}
