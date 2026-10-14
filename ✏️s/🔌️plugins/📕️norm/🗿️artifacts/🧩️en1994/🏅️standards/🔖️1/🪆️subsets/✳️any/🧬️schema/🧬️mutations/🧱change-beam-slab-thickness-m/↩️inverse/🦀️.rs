//! Inverse for `change-beam-slab-thickness-m`.
use crate::change_beam_slab_thickness_m::ChangeBeamSlabThicknessM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSlabThicknessM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_slab_thickness_m::ChangeBeamSlabThicknessM> {
    Ok((|| {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamSlabThicknessM { index: payload.index, new_slab_thickness_m: beam.slab_thickness_m })]

    })())
}
