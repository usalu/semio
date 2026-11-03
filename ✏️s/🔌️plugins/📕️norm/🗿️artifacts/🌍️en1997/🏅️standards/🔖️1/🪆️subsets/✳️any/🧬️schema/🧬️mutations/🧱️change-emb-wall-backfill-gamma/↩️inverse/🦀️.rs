//! ↩️ `change-emb-wall-backfill-gamma` inverse: restores the previous `backfill_gamma`.

use crate::change_emb_wall_backfill_gamma::ChangeEmbWallBackfillGamma;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallBackfillGamma, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_backfill_gamma::ChangeEmbWallBackfillGamma> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallBackfillGamma { id: payload.id.clone(), new_backfill_gamma: row.backfill_gamma })],
        None => Vec::new(),
    })
}
