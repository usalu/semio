//! 🆔️ `change-zone-id` diff — replaces the row by a copy under the new id at the same position; an id the document does not hold is a `mutation.invariant`, an id it already holds a `mutation.duplicate-id`. No other row names a zone, so nothing else follows.

use crate::change_zone_id::ChangeZoneId;
use semio_s_artifact_norm_din16798_diff::{Din16798Diff, Din16798ZoneDelta, Din16798ZonePatch};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

pub fn diff(payload: &ChangeZoneId, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some(index) = base.zones.iter().position(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    if payload.new_id == payload.zone_id {
        return protocol::MutationOutcome::new(Din16798Diff::default());
    }
    if base.zones.iter().any(|row| row.id == payload.new_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A zone with id '{}' already exists.", payload.new_id), [payload.new_id.clone()]);
    }
    let mut renamed = base.zones[index].clone();
    renamed.id = payload.new_id.clone();
    let mut zones = Din16798ZoneDelta::removal(&base.zones, index);
    zones.absorb(Din16798ZoneDelta::insertion(index, renamed));
    protocol::MutationOutcome::new(Din16798Diff { zones, ..Default::default() })
}
