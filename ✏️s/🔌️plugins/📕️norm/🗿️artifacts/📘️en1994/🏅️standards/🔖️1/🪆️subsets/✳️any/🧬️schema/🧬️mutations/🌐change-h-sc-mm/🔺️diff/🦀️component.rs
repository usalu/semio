//! 🔺️ `change-h-sc-mm` — sparse diff construction.

use super::mutation::ChangeHScMm;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeHScMm, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_h_sc_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "H sc mm must be a finite number.", Vec::<String>::new());
    }
    if base.h_sc_mm == payload.new_h_sc_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "H sc mm already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { h_sc_mm: Some(payload.new_h_sc_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
