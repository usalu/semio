//! 🔺️ `change-area-m2` — sparse diff construction.

use super::mutation::ChangeAreaM2;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAreaM2, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Area m2 must be a finite number.", Vec::<String>::new());
    }
    if base.area_m2 == payload.new_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Area m2 already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { area_m2: Some(payload.new_area_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
