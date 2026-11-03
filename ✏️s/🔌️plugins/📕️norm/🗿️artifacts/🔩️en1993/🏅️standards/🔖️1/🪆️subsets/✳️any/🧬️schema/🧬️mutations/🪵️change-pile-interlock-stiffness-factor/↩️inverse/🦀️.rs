//! ↩️ `change-pile-interlock-stiffness-factor` inverse: restores the previous `interlock_stiffness_factor`.

use crate::change_pile_interlock_stiffness_factor::ChangePileInterlockStiffnessFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileInterlockStiffnessFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_interlock_stiffness_factor::ChangePileInterlockStiffnessFactor> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileInterlockStiffnessFactor { id: payload.id.clone(), new_interlock_stiffness_factor: row.interlock_stiffness_factor })],
        None => Vec::new(),
    })
}
