//! ↩️ `change-cf-gross-resistance` inverse: restores the previous `gross_resistance`.

use crate::change_cf_gross_resistance::ChangeCfGrossResistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfGrossResistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_gross_resistance::ChangeCfGrossResistance> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfGrossResistance { id: payload.id.clone(), new_gross_resistance: row.gross_resistance })],
        None => Vec::new(),
    })
}
