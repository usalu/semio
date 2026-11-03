//! ↩️ `change-wall-backfill-gamma` inverse: restores the previous `backfill_gamma`.

use crate::change_wall_backfill_gamma::ChangeWallBackfillGamma;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallBackfillGamma, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_backfill_gamma::ChangeWallBackfillGamma> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallBackfillGamma { id: payload.id.clone(), new_backfill_gamma: row.backfill_gamma })],
        None => Vec::new(),
    })
}
