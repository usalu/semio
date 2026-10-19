//! ↩️ Inverse for `remove-self-weight-elements`.
use crate::remove_self_weight_elements::RemoveSelfWeightElements;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &RemoveSelfWeightElements, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_self_weight_elements::InsertSelfWeightElements> {
    Ok((|| {
    if payload.index >= base.self_weight_elements.len() { return Vec::new(); }
    let item = base.self_weight_elements[payload.index].clone();
    vec![Op::from(crate::insert_self_weight_elements::InsertSelfWeightElements { index: payload.index, item })]

    })())
}
