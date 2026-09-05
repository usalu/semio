//! 🔺️ `change-k-foundation` sparse diff construction — writes only `En1998Diff.k_foundation` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_k_foundation::ChangeKFoundation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeKFoundation, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_k_foundation.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Foundation stiffness k [kN/m] must be a finite number, got {}.", payload.new_k_foundation), Vec::<String>::new());
    }
    if base.k_foundation == payload.new_k_foundation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Foundation stiffness k [kN/m] is already {}.", payload.new_k_foundation));
    }
    protocol::MutationOutcome::new(En1998Diff { k_foundation: Some(payload.new_k_foundation.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
