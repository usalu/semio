//! ↩️ `change-forming-gain-ultimate-strength` inverse: restores the previous `ultimate_strength`.

use crate::change_forming_gain_ultimate_strength::ChangeFormingGainUltimateStrength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFormingGainUltimateStrength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_forming_gain_ultimate_strength::ChangeFormingGainUltimateStrength> {
    Ok(match base.forming_gains.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFormingGainUltimateStrength { id: payload.id.clone(), new_ultimate_strength: row.ultimate_strength })],
        None => Vec::new(),
    })
}
