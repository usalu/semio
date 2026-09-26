//! 🔺️ `change-delta-u-wb` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::change_delta_u_wb::ChangeDeltaUWb;
use crate::Din18599Snapshot;

pub fn diff(payload: &ChangeDeltaUWb, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_delta_u_wb_w_m2k.is_finite() || payload.new_delta_u_wb_w_m2k < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "delta-u-wb must be a non-negative finite number.", Vec::<String>::new());
    }
    if base.delta_u_wb_w_m2k == payload.new_delta_u_wb_w_m2k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "delta-u-wb already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { delta_u_wb_w_m2k: Some(payload.new_delta_u_wb_w_m2k), ..Default::default() })
}
