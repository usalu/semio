//! 🔺️ Sparse diff construction for the `rename-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::mutation::RenameEquipment;
use crate::artifacts::program::diff::{ProgramEquipmentDelta, ProgramEquipmentPatchEntry};
use crate::artifacts::program::registers::EquipmentPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameEquipment, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.equipment.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No equipment exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This equipment already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = EquipmentPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { equipment: Some(ProgramEquipmentDelta { patched: vec![ProgramEquipmentPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
