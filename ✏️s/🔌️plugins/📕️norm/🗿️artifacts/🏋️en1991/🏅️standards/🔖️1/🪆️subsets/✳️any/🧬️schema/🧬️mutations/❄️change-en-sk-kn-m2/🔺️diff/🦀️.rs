//! 🔺️ `change-en-sk-kn-m2` — sparse diff construction.

use super::ChangeEnSKKnM2;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnSKKnM2, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_en_s_k_kn_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "En sk kn m2 must be a finite number.", Vec::<String>::new());
    }
    if base.en_s_k_kn_m2 == payload.new_en_s_k_kn_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "En sk kn m2 already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { en_s_k_kn_m2: Some(payload.new_en_s_k_kn_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
