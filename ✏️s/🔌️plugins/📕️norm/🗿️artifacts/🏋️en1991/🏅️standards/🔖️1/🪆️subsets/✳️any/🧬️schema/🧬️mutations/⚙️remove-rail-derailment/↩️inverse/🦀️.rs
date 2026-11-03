//! ↩️ `remove-rail-derailment` inverse: inserts the removed row back at its position.

use crate::remove_rail_derailment::RemoveRailDerailment;
use crate::insert_rail_derailment::InsertRailDerailment;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveRailDerailment, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_rail_derailment::InsertRailDerailment> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveRailDerailment, base: &En1991Snapshot) -> Option<InsertRailDerailment> {
    let (index, row) = base.rail_derailments.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertRailDerailment { index, item: row.clone() })
}
