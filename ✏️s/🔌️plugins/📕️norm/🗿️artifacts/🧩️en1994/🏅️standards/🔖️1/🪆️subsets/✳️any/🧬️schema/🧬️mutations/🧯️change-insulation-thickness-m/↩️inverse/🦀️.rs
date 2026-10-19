//! ↩️ Inverse for `change-insulation-thickness-m`.
use crate::change_insulation_thickness_m::ChangeInsulationThicknessM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(_payload: &ChangeInsulationThicknessM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_insulation_thickness_m::ChangeInsulationThicknessM> {
    Ok((|| {
    vec![Op::from(ChangeInsulationThicknessM { new_insulation_thickness_m: base.insulation_thickness_m })]

    })())
}
