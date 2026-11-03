//! ↩️ Inverse for `change-beam-transverse-as-top`.
use crate::change_beam_transverse_as_top::ChangeBeamTransverseAsTop;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamTransverseAsTop, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_transverse_as_top::ChangeBeamTransverseAsTop> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamTransverseAsTop { id: payload.id.clone(), new_transverse_as_top_m2_per_m: beam.transverse_as_top_m2_per_m })])
}
