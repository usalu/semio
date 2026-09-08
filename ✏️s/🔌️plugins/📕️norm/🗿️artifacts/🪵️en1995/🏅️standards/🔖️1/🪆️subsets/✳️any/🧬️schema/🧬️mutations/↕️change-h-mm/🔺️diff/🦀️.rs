//! 🔺️ `change-h-mm` sparse diff construction — writes only `En1995Diff.h_mm` from the payload.

use crate::diff::En1995Diff;
use crate::mutations::change_h_mm::ChangeHMm;
use crate::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHMm, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_h_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "H mm must be a finite number.", Vec::<String>::new());
    }
    if base.h_mm == payload.new_h_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "H mm already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { h_mm: Some(payload.new_h_mm), ..Default::default() })
}
//#endregion 🔖️Diff
