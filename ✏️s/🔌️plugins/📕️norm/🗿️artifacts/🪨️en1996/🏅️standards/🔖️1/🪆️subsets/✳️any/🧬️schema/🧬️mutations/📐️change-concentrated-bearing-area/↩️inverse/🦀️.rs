use crate::change_concentrated_bearing_area::ChangeConcentratedBearingArea;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeConcentratedBearingArea, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_concentrated_bearing_area::ChangeConcentratedBearingArea> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index >= base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len() { Vec::new() } else { vec![Op::from(ChangeConcentratedBearingArea { wall_index: payload.wall_index, load_case_index: payload.load_case_index, index: payload.index, new_bearing_area_m2: base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated[payload.index].bearing_area_m2 })] }

    })())
}
