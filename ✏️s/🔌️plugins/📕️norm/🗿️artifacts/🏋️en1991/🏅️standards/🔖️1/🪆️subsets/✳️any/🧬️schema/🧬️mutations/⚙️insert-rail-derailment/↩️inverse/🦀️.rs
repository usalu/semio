//! ↩️ `insert-rail-derailment` inverse: removes the inserted row again.

use crate::insert_rail_derailment::InsertRailDerailment;
use crate::remove_rail_derailment::RemoveRailDerailment;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertRailDerailment, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_rail_derailment::RemoveRailDerailment> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertRailDerailment, base: &En1991Snapshot) -> Option<RemoveRailDerailment> {
    Some(RemoveRailDerailment { id: payload.item.id.clone() })
}
