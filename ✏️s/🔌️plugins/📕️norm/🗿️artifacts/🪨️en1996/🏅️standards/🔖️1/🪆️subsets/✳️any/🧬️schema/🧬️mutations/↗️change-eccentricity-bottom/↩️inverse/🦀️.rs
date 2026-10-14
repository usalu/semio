use crate::change_eccentricity_bottom::ChangeEccentricityBottom;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeEccentricityBottom, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_eccentricity_bottom::ChangeEccentricityBottom> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeEccentricityBottom { index: payload.index, new_eccentricity_bottom_m: base.walls[payload.index].eccentricity_bottom_m })] }

    })())
}
