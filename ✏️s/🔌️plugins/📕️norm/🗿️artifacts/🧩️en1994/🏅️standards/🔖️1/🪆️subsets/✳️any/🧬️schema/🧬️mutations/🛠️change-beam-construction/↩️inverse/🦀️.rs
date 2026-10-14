//! Inverse for `change-beam-construction`.
use crate::change_beam_construction::ChangeBeamConstruction;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamConstruction, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_construction::ChangeBeamConstruction> {
    Ok((|| {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamConstruction { index: payload.index, new_construction: beam.construction.clone() })]

    })())
}
