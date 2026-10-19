//! ↩️ Inverse for `insert-self-weight-elements`.
use crate::insert_self_weight_elements::InsertSelfWeightElements;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &InsertSelfWeightElements, _base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_self_weight_elements::RemoveSelfWeightElements> {
    Ok((|| {
    vec![Op::from(crate::remove_self_weight_elements::RemoveSelfWeightElements { index: payload.index })]

    })())
}
