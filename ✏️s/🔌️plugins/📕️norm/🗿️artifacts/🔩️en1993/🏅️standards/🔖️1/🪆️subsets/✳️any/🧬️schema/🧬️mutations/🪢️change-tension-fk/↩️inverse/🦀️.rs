//! ↩️ `change-tension-fk` inverse: restores the previous `f_k`.

use crate::change_tension_f_k::ChangeTensionFK;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionFK, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_f_k::ChangeTensionFK> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionFK { id: payload.id.clone(), new_f_k: row.f_k })],
        None => Vec::new(),
    })
}
