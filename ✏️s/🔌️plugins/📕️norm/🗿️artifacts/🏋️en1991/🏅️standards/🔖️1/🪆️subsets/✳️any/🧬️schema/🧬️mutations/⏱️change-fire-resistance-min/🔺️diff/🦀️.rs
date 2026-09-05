//! 🔺️ `change-fire-resistance-min` — sparse diff construction.

use super::ChangeFireResistanceMin;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireResistanceMin, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_fire_resistance_min.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Fire resistance min must be a finite number.", Vec::<String>::new());
    }
    if base.fire_resistance_min == payload.new_fire_resistance_min {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fire resistance min already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_resistance_min: Some(payload.new_fire_resistance_min.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
