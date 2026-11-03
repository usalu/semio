use crate::change_placement_x::ChangePlacementX;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePlacementX, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_x::ChangePlacementX> {
    Ok(match base.placements.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![Op::from(ChangePlacementX { id: payload.id.clone(), new_value: item.x })],
        None => Vec::new(),
    })
}
