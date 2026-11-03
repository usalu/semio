//! ↩️ `change-crane-flange-effective-width` inverse: restores the previous `flange_effective_width`.

use crate::change_crane_flange_effective_width::ChangeCraneFlangeEffectiveWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneFlangeEffectiveWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_flange_effective_width::ChangeCraneFlangeEffectiveWidth> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneFlangeEffectiveWidth { id: payload.id.clone(), new_flange_effective_width: row.flange_effective_width })],
        None => Vec::new(),
    })
}
