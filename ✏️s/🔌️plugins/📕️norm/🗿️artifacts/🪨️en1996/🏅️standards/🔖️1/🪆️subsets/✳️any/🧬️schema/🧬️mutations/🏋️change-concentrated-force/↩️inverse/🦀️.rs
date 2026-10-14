use crate::change_concentrated_force::ChangeConcentratedForce;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeConcentratedForce, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_concentrated_force::ChangeConcentratedForce> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index >= base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len() { Vec::new() } else { vec![Op::from(ChangeConcentratedForce { wall_index: payload.wall_index, load_case_index: payload.load_case_index, index: payload.index, new_force_n: base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated[payload.index].force_n })] }

    })())
}
