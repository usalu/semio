use crate::insert_zone::InsertZone;
use crate::remove_zone::RemoveZone;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &InsertZone, _base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_zone::RemoveZone> {
    Ok((|| {
    vec![Op::from(RemoveZone { zone_id: payload.zone.id.clone() })]

    })())
}
