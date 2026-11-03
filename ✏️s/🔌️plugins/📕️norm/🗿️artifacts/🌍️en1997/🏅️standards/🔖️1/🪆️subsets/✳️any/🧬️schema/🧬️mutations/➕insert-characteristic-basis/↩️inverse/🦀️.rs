use crate::insert_characteristic_basis::InsertCharacteristicBasis;
use crate::remove_characteristic_basis::RemoveCharacteristicBasis;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertCharacteristicBasis, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_characteristic_basis::RemoveCharacteristicBasis> {
    Ok((|| {
    let at = payload.index.min(base.characteristic_bases.len());
    vec![Op::from(RemoveCharacteristicBasis { index: at })]

    })())
}
