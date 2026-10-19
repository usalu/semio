//! ↩️ Inverse for `insert-beam`.
use crate::insert_beam::InsertBeam;
use crate::remove_beam::RemoveBeam;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &InsertBeam, _base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_beam::RemoveBeam> {
    Ok((|| {
    vec![Op::from(RemoveBeam { index: payload.index })]

    })())
}
