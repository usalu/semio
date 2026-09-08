//! 🔺️ `change-n-cycles-bridge` sparse diff construction — writes only `En1995Diff.n_cycles_bridge` from the payload.

use crate::diff::En1995Diff;
use crate::mutations::change_n_cycles_bridge::ChangeNCyclesBridge;
use crate::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNCyclesBridge, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_n_cycles_bridge.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "N cycles bridge must be a finite number.", Vec::<String>::new());
    }
    if base.n_cycles_bridge == payload.new_n_cycles_bridge {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "N cycles bridge already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { n_cycles_bridge: Some(payload.new_n_cycles_bridge), ..Default::default() })
}
//#endregion 🔖️Diff
