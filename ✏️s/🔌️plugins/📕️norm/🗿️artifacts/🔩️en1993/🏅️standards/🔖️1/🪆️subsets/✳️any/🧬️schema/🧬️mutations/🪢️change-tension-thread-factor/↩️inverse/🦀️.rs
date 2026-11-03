//! ↩️ `change-tension-thread-factor` inverse: restores the previous `thread_factor`.

use crate::change_tension_thread_factor::ChangeTensionThreadFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionThreadFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_thread_factor::ChangeTensionThreadFactor> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionThreadFactor { id: payload.id.clone(), new_thread_factor: row.thread_factor })],
        None => Vec::new(),
    })
}
