//! ↩️ Inverse for `change-storey-count`.
use crate::change_storey_count::ChangeStoreyCount;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeStoreyCount, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_count::ChangeStoreyCount> {
    Ok((|| {
    vec![Op::from(ChangeStoreyCount { new_storey_count: base.storey_count })]

    })())
}
