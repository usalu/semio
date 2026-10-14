//! ↩️ `change-zone-occupants` inverse.
use crate::change_zone_occupants::ChangeZoneOccupants;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneOccupants, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_occupants::ChangeZoneOccupants> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneOccupants { zone_id: payload.zone_id.clone(), new_occupants: z.occupants })]

    })())
}
