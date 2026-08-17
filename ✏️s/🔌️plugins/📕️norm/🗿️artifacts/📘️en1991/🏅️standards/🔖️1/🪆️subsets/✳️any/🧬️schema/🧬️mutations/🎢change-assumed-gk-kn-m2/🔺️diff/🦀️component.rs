//! 🔺️ `change-assumed-gk-kn-m2` — sparse diff construction.

use super::mutation::ChangeAssumedGKKnM2;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAssumedGKKnM2, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_g_k_kn_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Assumed gk kn m2 must be a finite number.", Vec::<String>::new());
    }
    if base.assumed_g_k_kn_m2 == payload.new_assumed_g_k_kn_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Assumed gk kn m2 already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_g_k_kn_m2: Some(payload.new_assumed_g_k_kn_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
