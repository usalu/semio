//! ↩️ `change-characteristic-basis-basis` inverse: restores the previous `basis`.

use crate::change_characteristic_basis_basis::ChangeCharacteristicBasisBasis;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeCharacteristicBasisBasis, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_characteristic_basis_basis::ChangeCharacteristicBasisBasis> {
    Ok(match base.characteristic_bases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCharacteristicBasisBasis { id: payload.id.clone(), new_basis: row.basis })],
        None => Vec::new(),
    })
}
