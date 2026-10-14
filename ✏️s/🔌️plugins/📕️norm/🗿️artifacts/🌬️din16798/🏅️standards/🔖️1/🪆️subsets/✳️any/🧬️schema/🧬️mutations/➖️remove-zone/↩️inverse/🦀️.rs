use crate::remove_zone::RemoveZone;
use crate::insert_zone::InsertZone;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &RemoveZone, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_zone::InsertZone> {
    Ok((|| {
    let Some((index, zone)) = base.zones.iter().enumerate().find(|(_, z)| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(InsertZone { index: Some(index), zone: zone.clone() })]

    })())
}
