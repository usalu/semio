//! ↩️ Inverse for `change-beam-steel-designation`: restores the designation and every geometry leaf, so a custom section returns exactly.
use crate::change_beam_steel_designation::ChangeBeamSteelDesignation;
use crate::change_beam_steel_height_m::ChangeBeamSteelHeightM;
use crate::change_beam_steel_width_m::ChangeBeamSteelWidthM;
use crate::change_beam_steel_web_thickness_m::ChangeBeamSteelWebThicknessM;
use crate::change_beam_steel_tf_m::ChangeBeamSteelTfM;
use crate::change_beam_steel_area_m2::ChangeBeamSteelAreaM2;
use crate::change_beam_steel_w_pl_y_m3::ChangeBeamSteelWPlYM3;
use crate::change_beam_steel_second_moment_m4::ChangeBeamSteelSecondMomentM4;
use crate::change_beam_steel_shear_area_m2::ChangeBeamSteelShearAreaM2;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSteelDesignation, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_steel_designation::ChangeBeamSteelDesignation> + From<crate::change_beam_steel_height_m::ChangeBeamSteelHeightM> + From<crate::change_beam_steel_width_m::ChangeBeamSteelWidthM> + From<crate::change_beam_steel_web_thickness_m::ChangeBeamSteelWebThicknessM> + From<crate::change_beam_steel_tf_m::ChangeBeamSteelTfM> + From<crate::change_beam_steel_area_m2::ChangeBeamSteelAreaM2> + From<crate::change_beam_steel_w_pl_y_m3::ChangeBeamSteelWPlYM3> + From<crate::change_beam_steel_second_moment_m4::ChangeBeamSteelSecondMomentM4> + From<crate::change_beam_steel_shear_area_m2::ChangeBeamSteelShearAreaM2> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![
        Op::from(ChangeBeamSteelDesignation { id: payload.id.clone(), new_designation: beam.steel.designation.clone() }),
        Op::from(ChangeBeamSteelHeightM { id: payload.id.clone(), new_height_m: beam.steel.height_m }),
        Op::from(ChangeBeamSteelWidthM { id: payload.id.clone(), new_width_m: beam.steel.width_m }),
        Op::from(ChangeBeamSteelWebThicknessM { id: payload.id.clone(), new_tw_m: beam.steel.tw_m }),
        Op::from(ChangeBeamSteelTfM { id: payload.id.clone(), new_tf_m: beam.steel.tf_m }),
        Op::from(ChangeBeamSteelAreaM2 { id: payload.id.clone(), new_a_m2: beam.steel.a_m2 }),
        Op::from(ChangeBeamSteelWPlYM3 { id: payload.id.clone(), new_w_pl_y_m3: beam.steel.w_pl_y_m3 }),
        Op::from(ChangeBeamSteelSecondMomentM4 { id: payload.id.clone(), new_i_y_m4: beam.steel.i_y_m4 }),
        Op::from(ChangeBeamSteelShearAreaM2 { id: payload.id.clone(), new_a_v_m2: beam.steel.a_v_m2 }),
    ])
}
