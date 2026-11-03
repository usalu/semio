//! ↩️ `change-emb-wall-phi-deg` inverse: restores the previous `phi_deg`.

use crate::change_emb_wall_phi_deg::ChangeEmbWallPhiDeg;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallPhiDeg, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_phi_deg::ChangeEmbWallPhiDeg> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallPhiDeg { id: payload.id.clone(), new_phi_deg: row.phi_deg })],
        None => Vec::new(),
    })
}
