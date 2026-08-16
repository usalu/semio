//! 🔺️ Sparse diff construction for the `create-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::mutation::CreateEquipment;
use crate::artifacts::program::diff::ProgramEquipmentDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.equipment` on apply.
pub fn diff(payload: &CreateEquipment, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { equipment: Some(ProgramEquipmentDelta { added: vec![payload.equipment.clone()], ..Default::default() }), ..Default::default() }
}
