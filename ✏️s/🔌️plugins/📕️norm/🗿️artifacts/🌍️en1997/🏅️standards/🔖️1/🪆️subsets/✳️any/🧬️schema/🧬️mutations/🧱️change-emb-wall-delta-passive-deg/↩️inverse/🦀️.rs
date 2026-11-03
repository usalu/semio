//! ↩️ `change-emb-wall-delta-passive-deg` inverse: restores the previous `delta_passive_deg`.

use crate::change_emb_wall_delta_passive_deg::ChangeEmbWallDeltaPassiveDeg;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallDeltaPassiveDeg, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_delta_passive_deg::ChangeEmbWallDeltaPassiveDeg> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallDeltaPassiveDeg { id: payload.id.clone(), new_delta_passive_deg: row.delta_passive_deg })],
        None => Vec::new(),
    })
}
