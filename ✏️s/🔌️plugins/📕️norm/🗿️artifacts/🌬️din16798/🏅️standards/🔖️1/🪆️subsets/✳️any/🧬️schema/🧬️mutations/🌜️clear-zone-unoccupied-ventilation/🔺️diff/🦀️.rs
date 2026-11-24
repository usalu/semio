//! 🌜️ `clear-zone-unoccupied-ventilation` diff - raises the clear flag of the row with that id; a zone without the record is a no-op, an id the document does not hold a `mutation.invariant`.

use crate::clear_zone_unoccupied_ventilation::ClearZoneUnoccupiedVentilation;
use semio_s_artifact_norm_din16798_diff::{Din16798Diff, Din16798ZoneDelta, Din16798ZonePatch};
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

pub fn diff(payload: &ClearZoneUnoccupiedVentilation, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    if row.unoccupied_ventilation.is_none() {
        return protocol::MutationOutcome::new(Din16798Diff::default());
    }
    let patch = Din16798ZonePatch { clear_unoccupied_ventilation: Some(true), ..Default::default() };
    protocol::MutationOutcome::new(Din16798Diff { zones: Din16798ZoneDelta::modification(&row.id, patch), ..Default::default() })
}
