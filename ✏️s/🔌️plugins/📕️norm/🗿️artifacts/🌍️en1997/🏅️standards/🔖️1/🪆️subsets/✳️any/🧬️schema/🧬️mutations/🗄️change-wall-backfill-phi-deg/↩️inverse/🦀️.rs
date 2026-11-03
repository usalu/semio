//! ↩️ `change-wall-backfill-phi-deg` inverse: restores the previous `backfill_phi_deg`.

use crate::change_wall_backfill_phi_deg::ChangeWallBackfillPhiDeg;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallBackfillPhiDeg, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_backfill_phi_deg::ChangeWallBackfillPhiDeg> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallBackfillPhiDeg { id: payload.id.clone(), new_backfill_phi_deg: row.backfill_phi_deg })],
        None => Vec::new(),
    })
}
