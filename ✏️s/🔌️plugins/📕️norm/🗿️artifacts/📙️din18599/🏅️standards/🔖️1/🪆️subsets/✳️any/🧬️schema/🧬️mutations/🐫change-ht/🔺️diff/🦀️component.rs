//! 🔺️ `change-ht` sparse diff construction — writes only `Din18599Diff.h_t` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_h_t::mutation::ChangeHT;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeHT, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_h_t.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Ht must be a finite number.", Vec::<String>::new());
    }
    if base.h_t == payload.new_h_t {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Ht already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { h_t: Some(payload.new_h_t.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
