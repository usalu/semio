//! ↩️ `change-pile-interlock-shear` inverse: restores the previous `interlock_shear`.

use crate::change_pile_interlock_shear::ChangePileInterlockShear;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileInterlockShear, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_interlock_shear::ChangePileInterlockShear> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileInterlockShear { id: payload.id.clone(), new_interlock_shear: row.interlock_shear })],
        None => Vec::new(),
    })
}
