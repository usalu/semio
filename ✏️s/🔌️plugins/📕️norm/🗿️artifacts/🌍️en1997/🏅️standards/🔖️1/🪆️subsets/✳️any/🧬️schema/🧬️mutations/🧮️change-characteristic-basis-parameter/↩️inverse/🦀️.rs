//! ↩️ `change-characteristic-basis-parameter` inverse: restores the previous `parameter`.

use crate::change_characteristic_basis_parameter::ChangeCharacteristicBasisParameter;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeCharacteristicBasisParameter, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_characteristic_basis_parameter::ChangeCharacteristicBasisParameter> {
    Ok(match base.characteristic_bases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCharacteristicBasisParameter { id: payload.id.clone(), new_parameter: row.parameter })],
        None => Vec::new(),
    })
}
