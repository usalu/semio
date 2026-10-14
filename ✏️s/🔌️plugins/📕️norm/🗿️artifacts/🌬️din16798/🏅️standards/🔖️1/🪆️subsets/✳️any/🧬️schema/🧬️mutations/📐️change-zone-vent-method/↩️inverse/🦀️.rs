//! ↩️ `change-zone-vent-method` inverse.
use crate::change_zone_vent_method::ChangeZoneVentMethod;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneVentMethod, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_vent_method::ChangeZoneVentMethod> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneVentMethod { zone_id: payload.zone_id.clone(), new_vent_method: z.vent_method.clone() })]

    })())
}
