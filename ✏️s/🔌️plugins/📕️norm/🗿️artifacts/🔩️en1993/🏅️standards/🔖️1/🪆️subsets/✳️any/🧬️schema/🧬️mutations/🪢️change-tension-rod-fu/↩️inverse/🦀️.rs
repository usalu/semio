//! ↩️ `change-tension-rod-fu` inverse: restores the previous `rod_fu`.

use crate::change_tension_rod_fu::ChangeTensionRodFu;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionRodFu, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_rod_fu::ChangeTensionRodFu> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionRodFu { id: payload.id.clone(), new_rod_fu: row.rod_fu })],
        None => Vec::new(),
    })
}
