//! 🔺️ `change-zone-label-en` sparse diff.

use crate::change_zone_label_en::ChangeZoneLabelEn;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ZonesRows, Din18599ZonesPatch};

pub fn diff(payload: &ChangeZoneLabelEn, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone \"{}\" does not exist.", payload.zone_id), [payload.zone_id.clone()]);
    };
    if row.label_en == payload.new_label_en {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Zone label en already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        zones: Some(Din18599ZonesRows::modification(&payload.zone_id, Din18599ZonesPatch { label_en: Some(payload.new_label_en.clone()), ..Default::default() })),
        ..Default::default()
    })
}
