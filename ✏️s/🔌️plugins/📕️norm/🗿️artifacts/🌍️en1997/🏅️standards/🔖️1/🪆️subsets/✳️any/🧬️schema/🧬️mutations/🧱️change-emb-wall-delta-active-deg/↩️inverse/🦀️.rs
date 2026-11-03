//! ↩️ `change-emb-wall-delta-active-deg` inverse: restores the previous `delta_active_deg`.

use crate::change_emb_wall_delta_active_deg::ChangeEmbWallDeltaActiveDeg;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallDeltaActiveDeg, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_delta_active_deg::ChangeEmbWallDeltaActiveDeg> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallDeltaActiveDeg { id: payload.id.clone(), new_delta_active_deg: row.delta_active_deg })],
        None => Vec::new(),
    })
}
