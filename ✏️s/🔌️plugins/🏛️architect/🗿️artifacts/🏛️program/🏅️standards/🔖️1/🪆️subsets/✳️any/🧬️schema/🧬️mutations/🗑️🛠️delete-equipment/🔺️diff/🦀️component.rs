//! 🔺️ Sparse diff construction for the `delete-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::mutation::DeleteEquipment;
use crate::artifacts::program::diff::ProgramEquipmentDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteEquipment, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { equipment: Some(ProgramEquipmentDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
