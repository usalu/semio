//! 🔺️ `change-element-zone-id` sparse diff.

use crate::change_element_zone_id::ChangeElementZoneId;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows, Din18599ElementsPatch};

pub fn diff(payload: &ChangeElementZoneId, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    let Some(row) = base.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.element_id), [payload.element_id.clone()]);
    };
    if row.zone_id == payload.new_zone_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Element zone id already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        elements: Some(Din18599ElementsRows::modification(&payload.element_id, Din18599ElementsPatch { zone_id: Some(payload.new_zone_id.clone()), ..Default::default() })),
        ..Default::default()
    })
}
