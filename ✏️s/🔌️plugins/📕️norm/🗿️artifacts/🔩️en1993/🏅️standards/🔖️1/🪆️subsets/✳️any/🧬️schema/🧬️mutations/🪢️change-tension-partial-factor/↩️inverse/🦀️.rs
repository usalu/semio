//! ↩️ `change-tension-partial-factor` inverse: restores the previous `partial_factor`.

use crate::change_tension_partial_factor::ChangeTensionPartialFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionPartialFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_partial_factor::ChangeTensionPartialFactor> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionPartialFactor { id: payload.id.clone(), new_partial_factor: row.partial_factor })],
        None => Vec::new(),
    })
}
