//! ↩️ `remove-zone` inverse: the row back at its position.

use crate::remove_zone::RemoveZone;
use crate::insert_zone::InsertZone;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &RemoveZone, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<InsertZone> {
    Ok(base.zones.iter().position(|row| row.id == payload.id).map(|index| vec![Op::from(InsertZone { index, zone: base.zones[index].clone() })]).unwrap_or_default())
}
