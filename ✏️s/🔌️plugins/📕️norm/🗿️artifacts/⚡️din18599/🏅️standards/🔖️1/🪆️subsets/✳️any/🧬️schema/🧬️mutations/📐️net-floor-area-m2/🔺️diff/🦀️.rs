//! 🔺️ `change-net-floor-area-m2` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::change_net_floor_area_m2::ChangeNetFloorAreaM2;
use crate::Din18599Snapshot;

pub fn diff(payload: &ChangeNetFloorAreaM2, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_net_floor_area_m2.is_finite() || payload.new_net_floor_area_m2 <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "net-floor-area must be a positive finite number.", Vec::<String>::new());
    }
    if base.net_floor_area_m2 == payload.new_net_floor_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "net-floor-area already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { net_floor_area_m2: Some(payload.new_net_floor_area_m2), ..Default::default() })
}
