use crate::change_pile_length::ChangePileLength;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &ChangePileLength, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_length::ChangePileLength> {
    Ok((|| {
    let length = base.piles.iter().find(|f| f.id == payload.id).map(|f| f.length).unwrap_or(payload.new_length);
    vec![Op::from(ChangePileLength { id: payload.id.clone(), new_length: length })]

    })())
}
