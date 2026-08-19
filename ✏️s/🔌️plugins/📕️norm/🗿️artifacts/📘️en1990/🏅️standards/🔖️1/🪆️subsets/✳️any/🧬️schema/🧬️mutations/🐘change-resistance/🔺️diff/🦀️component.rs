//! 🔺️ `change-resistance` — sparse diff construction; writes only `En1990Diff.resistance_kn`.

use super::mutation::ChangeResistance;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeResistance, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_resistance_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Resistance R_d must be a finite number.", Vec::<String>::new());
    }
    if base.resistance_kn == payload.new_resistance_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Resistance already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { resistance_kn: Some(payload.new_resistance_kn), ..Default::default() })
}
//#endregion 🔖️Diff
