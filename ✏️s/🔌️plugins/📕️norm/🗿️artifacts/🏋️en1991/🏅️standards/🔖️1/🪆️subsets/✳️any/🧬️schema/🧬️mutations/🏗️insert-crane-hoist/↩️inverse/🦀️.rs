//! ↩️ `insert-crane-hoist` inverse: removes the inserted row again.

use crate::insert_crane_hoist::InsertCraneHoist;
use crate::remove_crane_hoist::RemoveCraneHoist;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertCraneHoist, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_crane_hoist::RemoveCraneHoist> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertCraneHoist, base: &En1991Snapshot) -> Option<RemoveCraneHoist> {
    Some(RemoveCraneHoist { id: payload.item.id.clone() })
}
