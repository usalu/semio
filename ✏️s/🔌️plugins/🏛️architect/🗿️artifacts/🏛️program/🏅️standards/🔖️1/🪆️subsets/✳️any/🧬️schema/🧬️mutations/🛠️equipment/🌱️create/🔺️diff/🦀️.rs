//! 🔺️ Sparse diff construction for the `create-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::CreateEquipment;
use crate::diff::ProgramEquipmentDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateEquipment, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.equipment.header.id;
    if base.equipment.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An equipment already exists with this id.", [id.0.clone()]);
    }
    let length = base.equipment.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the equipment list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.equipment.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { equipment: Some(ProgramEquipmentDelta { added: vec![payload.equipment.clone()], reordered, ..Default::default() }), ..Default::default() })
}
