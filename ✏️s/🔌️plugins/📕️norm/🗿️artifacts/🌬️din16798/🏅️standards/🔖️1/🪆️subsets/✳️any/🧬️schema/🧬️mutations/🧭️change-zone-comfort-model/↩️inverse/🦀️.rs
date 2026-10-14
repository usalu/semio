//! ↩️ `change-zone-comfort-model` inverse.
use crate::change_zone_comfort_model::ChangeZoneComfortModel;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneComfortModel, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_comfort_model::ChangeZoneComfortModel> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneComfortModel { zone_id: payload.zone_id.clone(), new_comfort_model: z.comfort_model.clone() })]

    })())
}
