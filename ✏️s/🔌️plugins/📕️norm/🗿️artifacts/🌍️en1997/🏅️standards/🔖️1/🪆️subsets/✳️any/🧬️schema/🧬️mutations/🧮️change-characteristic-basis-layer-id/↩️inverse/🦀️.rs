//! ↩️ `change-characteristic-basis-layer-id` inverse: restores the previous `layer_id`.

use crate::change_characteristic_basis_layer_id::ChangeCharacteristicBasisLayerId;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeCharacteristicBasisLayerId, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_characteristic_basis_layer_id::ChangeCharacteristicBasisLayerId> {
    Ok(match base.characteristic_bases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCharacteristicBasisLayerId { id: payload.id.clone(), new_layer_id: row.layer_id.clone() })],
        None => Vec::new(),
    })
}
