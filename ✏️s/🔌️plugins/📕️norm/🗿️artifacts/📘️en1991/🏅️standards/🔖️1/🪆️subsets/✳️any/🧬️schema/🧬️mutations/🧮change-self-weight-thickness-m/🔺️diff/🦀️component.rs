//! 🔺️ `change-self-weight-thickness-m` — sparse diff construction.

use super::mutation::ChangeSelfWeightThicknessM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSelfWeightThicknessM, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_self_weight_thickness_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Self weight thickness m must be a finite number.", Vec::<String>::new());
    }
    if base.self_weight_thickness_m == payload.new_self_weight_thickness_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Self weight thickness m already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { self_weight_thickness_m: Some(payload.new_self_weight_thickness_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
