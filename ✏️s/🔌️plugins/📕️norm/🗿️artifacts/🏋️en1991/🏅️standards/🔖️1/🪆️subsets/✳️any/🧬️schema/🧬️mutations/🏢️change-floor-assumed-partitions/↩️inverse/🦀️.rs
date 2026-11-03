//! ↩️ `change-floor-assumed-partitions` inverse: restores the previous `assumed_partitions`.

use crate::change_floor_assumed_partitions::ChangeFloorAssumedPartitions;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFloorAssumedPartitions, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_assumed_partitions::ChangeFloorAssumedPartitions> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFloorAssumedPartitions, base: &En1991Snapshot) -> Option<ChangeFloorAssumedPartitions> {
    let r0 = base.floors.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFloorAssumedPartitions { id: payload.id.clone(), new_assumed_partitions: r0.assumed_partitions })
}
