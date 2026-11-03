//! ↩️ `change-forming-gain-gross-area` inverse: restores the previous `gross_area`.

use crate::change_forming_gain_gross_area::ChangeFormingGainGrossArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFormingGainGrossArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_forming_gain_gross_area::ChangeFormingGainGrossArea> {
    Ok(match base.forming_gains.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFormingGainGrossArea { id: payload.id.clone(), new_gross_area: row.gross_area })],
        None => Vec::new(),
    })
}
