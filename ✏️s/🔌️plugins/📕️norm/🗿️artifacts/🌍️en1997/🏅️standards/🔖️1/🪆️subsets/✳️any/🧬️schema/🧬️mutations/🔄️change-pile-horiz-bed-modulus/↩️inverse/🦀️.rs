//! ↩️ `change-pile-horiz-bed-modulus` inverse: restores the previous `horizontal_subgrade_modulus`.

use crate::change_pile_horiz_bed_modulus::ChangePileHorizBedModulus;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileHorizBedModulus, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_horiz_bed_modulus::ChangePileHorizBedModulus> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileHorizBedModulus { id: payload.id.clone(), new_horizontal_subgrade_modulus: row.horizontal_subgrade_modulus })],
        None => Vec::new(),
    })
}
