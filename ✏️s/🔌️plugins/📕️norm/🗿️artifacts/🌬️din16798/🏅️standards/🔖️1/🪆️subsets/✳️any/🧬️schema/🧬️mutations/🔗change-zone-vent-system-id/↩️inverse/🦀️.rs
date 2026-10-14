//! ↩️ `change-zone-vent-system-id` inverse.
use crate::change_zone_vent_system_id::ChangeZoneVentSystemId;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneVentSystemId, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_vent_system_id::ChangeZoneVentSystemId> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneVentSystemId { zone_id: payload.zone_id.clone(), new_vent_system_id: z.vent_system_id.clone() })]

    })())
}
