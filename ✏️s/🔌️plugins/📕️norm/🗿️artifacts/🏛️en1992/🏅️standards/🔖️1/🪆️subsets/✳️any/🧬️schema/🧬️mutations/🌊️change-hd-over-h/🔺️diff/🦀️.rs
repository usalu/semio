//! 🔺️ `change-hd-over-h` sparse diff construction — writes only `En1992Diff.hd_over_h` from the payload.

use crate::diff::En1992Diff;
use crate::mutations::change_hd_over_h::ChangeHdOverH;
use crate::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHdOverH, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_hd_over_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Hd over h must be a finite number.", Vec::<String>::new());
    }
    if base.hd_over_h == payload.new_hd_over_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Hd over h already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { hd_over_h: Some(payload.new_hd_over_h), ..Default::default() })
}
//#endregion 🔖️Diff
