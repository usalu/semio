use crate::change_placement_z::ChangePlacementZ;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangePlacementZ, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_z::ChangePlacementZ> {
    Ok(match base.placements.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![Op::from(ChangePlacementZ { id: payload.id.clone(), new_value: item.z })],
        None => Vec::new(),
    })
}
