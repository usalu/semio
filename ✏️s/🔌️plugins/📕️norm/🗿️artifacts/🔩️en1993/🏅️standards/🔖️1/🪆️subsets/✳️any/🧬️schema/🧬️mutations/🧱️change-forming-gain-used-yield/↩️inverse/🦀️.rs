//! ↩️ `change-forming-gain-used-yield` inverse: restores the previous `used_yield`.

use crate::change_forming_gain_used_yield::ChangeFormingGainUsedYield;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFormingGainUsedYield, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_forming_gain_used_yield::ChangeFormingGainUsedYield> {
    Ok(match base.forming_gains.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFormingGainUsedYield { id: payload.id.clone(), new_used_yield: row.used_yield })],
        None => Vec::new(),
    })
}
