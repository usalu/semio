//! ↩️ `change-weld-steel-fu` inverse: restores the previous `steel_fu`.

use crate::change_weld_steel_fu::ChangeWeldSteelFu;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldSteelFu, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_steel_fu::ChangeWeldSteelFu> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldSteelFu { id: payload.id.clone(), new_steel_fu: row.steel_fu })],
        None => Vec::new(),
    })
}
