use crate::change_load_case_situation::ChangeLoadCaseSituation;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeLoadCaseSituation, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_load_case_situation::ChangeLoadCaseSituation> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.load_case_index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![Op::from(ChangeLoadCaseSituation {
            wall_index: payload.wall_index,
            load_case_index: payload.load_case_index,
            new_design_situation: base.walls[payload.wall_index].load_cases[payload.load_case_index].design_situation.clone(),
        })]
    }

    })())
}
