//! ↩️ Inverse for `change-roof-assumed-sk`.
use crate::change_roof_assumed_sk::ChangeRoofAssumedSk;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(payload: &ChangeRoofAssumedSk, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_roof_assumed_sk::ChangeRoofAssumedSk> {
    Ok((|| {
    if payload.index >= base.roofs.len() { return Vec::new(); }
    vec![Op::from(ChangeRoofAssumedSk { index: payload.index, new_assumed_sk: base.roofs[payload.index].assumed_sk })]

    })())
}
