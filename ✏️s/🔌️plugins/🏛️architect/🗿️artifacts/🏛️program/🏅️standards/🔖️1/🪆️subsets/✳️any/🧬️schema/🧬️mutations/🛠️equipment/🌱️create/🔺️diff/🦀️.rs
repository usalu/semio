//! 🔺️ Sparse diff construction for the `create-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::CreateEquipment;
use crate::diff::ProgramEquipmentDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateEquipment, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.equipment.header.id;
    if base.equipment.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An equipment already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.equipment.len());
    if at > base.equipment.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the equipment list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { equipment: Some(ProgramEquipmentDelta::insertion(at, payload.equipment.clone())), ..Default::default() })
}
