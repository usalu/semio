//! ↩️ Inverse for `change-beam-construction`.
use crate::change_beam_construction::ChangeBeamConstruction;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamConstruction, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_construction::ChangeBeamConstruction> {
    Ok((|| {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Vec::new();
    };
    let Some(beam) = base.beams.get(index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamConstruction { id: payload.id.clone(), new_construction: beam.construction })]

    })())
}
