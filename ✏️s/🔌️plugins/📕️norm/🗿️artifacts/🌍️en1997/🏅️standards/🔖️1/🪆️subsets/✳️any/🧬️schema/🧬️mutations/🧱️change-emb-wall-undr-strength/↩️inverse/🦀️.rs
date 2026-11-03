//! ↩️ `change-emb-wall-undr-strength` inverse: restores the previous `undrained_strength`.

use crate::change_emb_wall_undr_strength::ChangeEmbWallUndrStrength;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallUndrStrength, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_undr_strength::ChangeEmbWallUndrStrength> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallUndrStrength { id: payload.id.clone(), new_undrained_strength: row.undrained_strength })],
        None => Vec::new(),
    })
}
