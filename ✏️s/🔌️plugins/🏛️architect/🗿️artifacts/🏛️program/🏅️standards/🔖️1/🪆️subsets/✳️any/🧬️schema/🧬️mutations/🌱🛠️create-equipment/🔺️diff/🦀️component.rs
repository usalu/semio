//! 🔺️ Sparse diff construction for the `create-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::mutation::CreateEquipment;
use crate::artifacts::program::diff::ProgramEquipmentDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateEquipment, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.equipment.header.id.clone();
    if base.equipment.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An equipment already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { equipment: Some(ProgramEquipmentDelta { added: vec![payload.equipment.clone()], ..Default::default() }), ..Default::default() })
}
