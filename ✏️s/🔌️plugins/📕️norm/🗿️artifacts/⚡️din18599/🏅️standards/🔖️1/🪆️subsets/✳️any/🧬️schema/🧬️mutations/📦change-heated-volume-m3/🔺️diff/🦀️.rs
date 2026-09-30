//! 🔺️ `change-heated-volume-m3` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::change_heated_volume_m3::ChangeHeatedVolumeM3;
use crate::Din18599Snapshot;

pub fn diff(payload: &ChangeHeatedVolumeM3, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_heated_volume_m3.is_finite() || payload.new_heated_volume_m3 <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "heated-volume must be a positive finite number.", Vec::<String>::new());
    }
    if base.heated_volume_m3 == payload.new_heated_volume_m3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "heated-volume already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { heated_volume_m3: Some(payload.new_heated_volume_m3), ..Default::default() })
}
