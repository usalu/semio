//! ↩️ `change-forming-gain-bends` inverse: restores the previous `bends`.

use crate::change_forming_gain_bends::ChangeFormingGainBends;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFormingGainBends, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_forming_gain_bends::ChangeFormingGainBends> {
    Ok(match base.forming_gains.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFormingGainBends { id: payload.id.clone(), new_bends: row.bends })],
        None => Vec::new(),
    })
}
