//! ↩️ Inverse for `remove-beam`.
use crate::remove_beam::RemoveBeam;
use crate::insert_beam::InsertBeam;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &RemoveBeam, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_beam::InsertBeam> {
    Ok((|| {
    let Some(beam) = base.beams.get(payload.index).cloned() else { return Vec::new(); };
    vec![Op::from(InsertBeam { index: payload.index, beam })]

    })())
}
