//! ↩️ `insert-zone` inverse: the removal of the inserted row.

use crate::insert_zone::InsertZone;
use crate::remove_zone::RemoveZone;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &InsertZone, _base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<RemoveZone> {
    Ok(vec![Op::from(RemoveZone { id: payload.zone.id.clone() })])
}
