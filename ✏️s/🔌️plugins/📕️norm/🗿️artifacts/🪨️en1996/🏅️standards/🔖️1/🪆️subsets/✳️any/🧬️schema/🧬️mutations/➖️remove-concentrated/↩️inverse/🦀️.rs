use crate::remove_concentrated::RemoveConcentrated;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use crate::insert_concentrated;
pub fn inverse<Op>(payload: &RemoveConcentrated, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_concentrated::InsertConcentrated> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() || payload.index >= base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated.len() { Vec::new() } else { vec![Op::from(insert_concentrated::InsertConcentrated { wall_index: payload.wall_index, load_case_index: payload.load_case_index, index: payload.index, load: base.walls[payload.wall_index].load_cases[payload.load_case_index].concentrated[payload.index].clone() })] }

    })())
}
