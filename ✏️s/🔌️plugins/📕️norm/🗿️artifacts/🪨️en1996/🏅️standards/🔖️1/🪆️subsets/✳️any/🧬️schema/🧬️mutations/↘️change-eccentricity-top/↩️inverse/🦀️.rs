use crate::change_eccentricity_top::ChangeEccentricityTop;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeEccentricityTop, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_eccentricity_top::ChangeEccentricityTop> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeEccentricityTop { index: payload.index, new_eccentricity_top_m: base.walls[payload.index].eccentricity_top_m })] }

    })())
}
