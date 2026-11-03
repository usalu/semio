//! ↩️ `change-tension-f-uk` inverse: restores the previous `f_uk`.

use crate::change_tension_f_uk::ChangeTensionFUk;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionFUk, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_f_uk::ChangeTensionFUk> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionFUk { id: payload.id.clone(), new_f_uk: row.f_uk })],
        None => Vec::new(),
    })
}
