//! 🔺️ `change-insulation-thickness-mm` — sparse diff construction.

use super::mutation::ChangeInsulationThicknessMm;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeInsulationThicknessMm, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_insulation_thickness_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Insulation thickness mm must be a finite number.", Vec::<String>::new());
    }
    if base.insulation_thickness_mm == payload.new_insulation_thickness_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Insulation thickness mm already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { insulation_thickness_mm: Some(payload.new_insulation_thickness_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
