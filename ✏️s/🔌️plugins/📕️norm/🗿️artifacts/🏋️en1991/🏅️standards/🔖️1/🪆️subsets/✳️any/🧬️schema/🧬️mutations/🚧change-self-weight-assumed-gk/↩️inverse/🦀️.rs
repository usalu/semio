//! Inverse for `change-self-weight-assumed-gk`.
use crate::change_self_weight_assumed_gk::ChangeSelfWeightAssumedGk;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &ChangeSelfWeightAssumedGk, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_self_weight_assumed_gk::ChangeSelfWeightAssumedGk> {
    Ok((|| {
    if payload.index >= base.self_weight_elements.len() { return Vec::new(); }
    vec![Op::from(ChangeSelfWeightAssumedGk { index: payload.index, new_assumed_gk: base.self_weight_elements[payload.index].assumed_gk })]

    })())
}
