//! ↩️ `change-pile-ilock-bending-factor` inverse: restores the previous `interlock_bending_factor`.

use crate::change_pile_ilock_bending_factor::ChangePileIlockBendingFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileIlockBendingFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_ilock_bending_factor::ChangePileIlockBendingFactor> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileIlockBendingFactor { id: payload.id.clone(), new_interlock_bending_factor: row.interlock_bending_factor })],
        None => Vec::new(),
    })
}
