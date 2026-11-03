//! ↩️ `remove-crane-hoist` inverse: inserts the removed row back at its position.

use crate::remove_crane_hoist::RemoveCraneHoist;
use crate::insert_crane_hoist::InsertCraneHoist;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveCraneHoist, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_crane_hoist::InsertCraneHoist> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveCraneHoist, base: &En1991Snapshot) -> Option<InsertCraneHoist> {
    let (index, row) = base.crane_hoists.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertCraneHoist { index, item: row.clone() })
}
