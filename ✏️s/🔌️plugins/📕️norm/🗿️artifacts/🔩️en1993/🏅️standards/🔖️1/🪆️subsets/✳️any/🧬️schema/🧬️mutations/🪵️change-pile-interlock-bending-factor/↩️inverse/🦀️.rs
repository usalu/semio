//! ↩️ `change-pile-interlock-bending-factor` inverse: restores the previous `interlock_bending_factor`.

use crate::change_pile_interlock_bending_factor::ChangePileInterlockBendingFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileInterlockBendingFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_interlock_bending_factor::ChangePileInterlockBendingFactor> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileInterlockBendingFactor { id: payload.id.clone(), new_interlock_bending_factor: row.interlock_bending_factor })],
        None => Vec::new(),
    })
}
