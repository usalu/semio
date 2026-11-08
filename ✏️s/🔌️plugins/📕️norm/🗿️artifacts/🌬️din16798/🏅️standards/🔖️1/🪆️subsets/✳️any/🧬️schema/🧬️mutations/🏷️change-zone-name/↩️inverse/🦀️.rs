//! ↩️ `change-zone-name` inverse.
use crate::change_zone_name::ChangeZoneName;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneName, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_name::ChangeZoneName> {
    Ok((|| {
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneName { zone_id: payload.zone_id.clone(), new_name: row.name.clone() })]

    })())
}
