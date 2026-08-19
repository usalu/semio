//! 🔺️ Sparse diff construction for the `delete-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::mutation::DeleteEquipment;
use crate::artifacts::program::diff::ProgramEquipmentDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteEquipment, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.equipment.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No equipment exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { equipment: Some(ProgramEquipmentDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
