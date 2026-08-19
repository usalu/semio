//! 🔺️ `change-k-soil` sparse diff construction — writes only `En1998Diff.k_soil` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_k_soil::mutation::ChangeKSoil;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeKSoil, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_k_soil.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Soil stiffness k [kN/m] must be a finite number, got {}.", payload.new_k_soil), Vec::<String>::new());
    }
    if base.k_soil == payload.new_k_soil {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Soil stiffness k [kN/m] is already {}.", payload.new_k_soil));
    }
    protocol::MutationOutcome::new(En1998Diff { k_soil: Some(payload.new_k_soil.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
