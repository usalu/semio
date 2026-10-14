use crate::change_tributary_area::ChangeTributaryArea;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeTributaryArea, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tributary_area::ChangeTributaryArea> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![Op::from(ChangeTributaryArea {
            wall_index: payload.wall_index,
            index: payload.index,
            new_tributary_area_m2: base.walls[payload.wall_index].load_cases[payload.index].tributary_area_m2,
        })]
    }

    })())
}
