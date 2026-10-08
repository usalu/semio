//! 💨 `change-zone-air-speed` diff — patches the one field of the row with that id; an id the document does not hold is a `mutation.invariant`.

use super::ChangeZoneAirSpeed;
use crate::diff::Din16798RowEdit as _;
use crate::diff::{Din16798Diff, Din16798ZoneEdit, Din16798ZonePatch};
use crate::Din16798Snapshot;

pub fn diff(payload: &ChangeZoneAirSpeed, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    let Some((index, row)) = base.zones.iter().enumerate().find(|(_, row)| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "zone not found", Vec::<String>::new());
    };
    let patch = Din16798ZonePatch { air_speed_m_s: Some(payload.new_air_speed_m_s), ..Default::default() };
    protocol::MutationOutcome::new(Din16798Diff { zones: vec![Din16798ZoneEdit::patch(index, row.id.clone(), patch)], ..Default::default() })
}
