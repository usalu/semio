//! 🔺️ `change-hv` sparse diff construction — writes only `Din18599Diff.h_v` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_h_v::mutation::ChangeHV;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHV, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_h_v.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Hv must be a finite number.", Vec::<String>::new());
    }
    if base.h_v == payload.new_h_v {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Hv already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { h_v: Some(payload.new_h_v.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
