//! ↩️ `change-crane-torsion-constant` inverse: restores the previous `torsion_constant`.

use crate::change_crane_torsion_constant::ChangeCraneTorsionConstant;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneTorsionConstant, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_torsion_constant::ChangeCraneTorsionConstant> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneTorsionConstant { id: payload.id.clone(), new_torsion_constant: row.torsion_constant })],
        None => Vec::new(),
    })
}
