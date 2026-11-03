//! ↩️ `change-cf-moment-resistance` inverse: restores the previous `moment_resistance`.

use crate::change_cf_moment_resistance::ChangeCfMomentResistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfMomentResistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_moment_resistance::ChangeCfMomentResistance> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfMomentResistance { id: payload.id.clone(), new_moment_resistance: row.moment_resistance })],
        None => Vec::new(),
    })
}
