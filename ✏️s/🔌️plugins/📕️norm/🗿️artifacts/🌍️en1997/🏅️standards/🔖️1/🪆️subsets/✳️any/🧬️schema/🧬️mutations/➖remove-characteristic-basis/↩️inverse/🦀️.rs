use crate::remove_characteristic_basis::RemoveCharacteristicBasis;
use crate::insert_characteristic_basis::InsertCharacteristicBasis;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveCharacteristicBasis, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_characteristic_basis::InsertCharacteristicBasis> {
    Ok((|| {
    match base.characteristic_bases.get(payload.index).cloned() {
        Some(characteristic_basis) => vec![Op::from(InsertCharacteristicBasis { index: payload.index, characteristic_basis })],
        None => Vec::new(),
    }

    })())
}
