use crate::change_imposed_category::ChangeImposedCategory;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeImposedCategory, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_imposed_category::ChangeImposedCategory> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() {
        Vec::new()
    } else {
        vec![Op::from(ChangeImposedCategory {
            wall_index: payload.wall_index,
            index: payload.index,
            new_imposed_category: base.walls[payload.wall_index].load_cases[payload.index].imposed_category.clone(),
        })]
    }

    })())
}
