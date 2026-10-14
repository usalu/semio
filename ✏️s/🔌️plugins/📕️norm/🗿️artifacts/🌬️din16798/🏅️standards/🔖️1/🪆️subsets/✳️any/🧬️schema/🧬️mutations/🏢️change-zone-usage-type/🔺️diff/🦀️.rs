//! 🏢️ `change-zone-usage-type` diff — patches the one field of the row with that id; an id the document does not hold is a `mutation.invariant`.

use crate::change_zone_usage_type::ChangeZoneUsageType;
use semio_s_artifact_norm_din16798_diff::{Din16798Diff, Din16798ZoneDelta, Din16798ZonePatch};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

pub fn diff(payload: &ChangeZoneUsageType, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    let patch = Din16798ZonePatch { usage_type: Some(payload.new_usage_type.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Din16798Diff { zones: Din16798ZoneDelta::modification(&row.id, patch), ..Default::default() })
}
