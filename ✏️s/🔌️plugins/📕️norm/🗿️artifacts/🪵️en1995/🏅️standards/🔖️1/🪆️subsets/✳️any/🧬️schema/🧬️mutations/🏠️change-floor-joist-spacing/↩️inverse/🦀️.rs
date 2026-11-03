use crate::change_floor_joist_spacing::ChangeFloorJoistSpacing;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFloorJoistSpacing, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_joist_spacing::ChangeFloorJoistSpacing> {
    Ok(match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => vec![Op::from(ChangeFloorJoistSpacing { floor_id: payload.floor_id.clone(), new_value: item.joist_spacing_m })],
        None => Vec::new(),
    })
}
