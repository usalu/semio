//! 🔺️ `change-structural-system` sparse diff construction — writes only `En1998Diff.structural_system` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_structural_system::ChangeStructuralSystem;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStructuralSystem, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.structural_system == payload.new_structural_system {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Structural system is already \"{}\".", payload.new_structural_system));
    }
    protocol::MutationOutcome::new(En1998Diff { structural_system: Some(payload.new_structural_system.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
