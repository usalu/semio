//! ↩️ `change-forming-gain-forming` inverse: restores the previous `forming`.

use crate::change_forming_gain_forming::ChangeFormingGainForming;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFormingGainForming, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_forming_gain_forming::ChangeFormingGainForming> {
    Ok(match base.forming_gains.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFormingGainForming { id: payload.id.clone(), new_forming: row.forming })],
        None => Vec::new(),
    })
}
